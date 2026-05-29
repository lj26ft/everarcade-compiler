use crate::stable_hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrushType {
    Circle,
    Square,
    Falloff,
    Custom(Vec<(i32, i32, i32)>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerrainTool {
    Raise,
    Lower,
    Smooth,
    Flatten(i32),
    Paint(&'static str),
    Noise { seed: u64, amplitude: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainBrush {
    pub brush_type: BrushType,
    pub radius: i32,
    pub strength: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionPaintKind {
    Region,
    Partition,
    Biome,
    CivilizationTerritory,
    ResourceZone,
    SpawnZone,
}

impl RegionPaintKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Region => "region",
            Self::Partition => "partition",
            Self::Biome => "biome",
            Self::CivilizationTerritory => "civilization-territory",
            Self::ResourceZone => "resource-zone",
            Self::SpawnZone => "spawn-zone",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainCell {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub paint: String,
    pub metadata: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerrainSurface {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<TerrainCell>,
    pub deterministic_seed: u64,
    pub terrain_hash: String,
}

impl TerrainSurface {
    pub fn new(width: i32, height: i32, deterministic_seed: u64) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(TerrainCell {
                    x,
                    y,
                    height: 0,
                    paint: "grass".into(),
                    metadata: Vec::new(),
                });
            }
        }
        let mut surface = Self {
            width,
            height,
            cells,
            deterministic_seed,
            terrain_hash: String::new(),
        };
        surface.rebuild_hash();
        surface
    }

    pub fn apply(&mut self, tool: TerrainTool, brush: &TerrainBrush, cx: i32, cy: i32) {
        let affected = self.affected_cells(brush, cx, cy);
        for index in affected {
            let weight = self.brush_weight(brush, self.cells[index].x, self.cells[index].y, cx, cy);
            match &tool {
                TerrainTool::Raise => self.cells[index].height += brush.strength * weight,
                TerrainTool::Lower => self.cells[index].height -= brush.strength * weight,
                TerrainTool::Smooth => {
                    self.cells[index].height = (self.cells[index].height + weight) / 2;
                }
                TerrainTool::Flatten(level) => self.cells[index].height = *level,
                TerrainTool::Paint(material) => self.cells[index].paint = (*material).to_owned(),
                TerrainTool::Noise { seed, amplitude } => {
                    self.cells[index].height += deterministic_noise(
                        *seed ^ self.deterministic_seed,
                        self.cells[index].x,
                        self.cells[index].y,
                    ) % amplitude.max(&1)
                }
            }
        }
        self.rebuild_hash();
    }

    pub fn paint_metadata(
        &mut self,
        kind: RegionPaintKind,
        label: &str,
        cx: i32,
        cy: i32,
        radius: i32,
    ) {
        let brush = TerrainBrush {
            brush_type: BrushType::Circle,
            radius,
            strength: 1,
        };
        let entry = format!("{}:{label}", kind.as_str());
        for index in self.affected_cells(&brush, cx, cy) {
            if !self.cells[index]
                .metadata
                .iter()
                .any(|existing| existing == &entry)
            {
                self.cells[index].metadata.push(entry.clone());
                self.cells[index].metadata.sort();
            }
        }
        self.rebuild_hash();
    }

    fn affected_cells(&self, brush: &TerrainBrush, cx: i32, cy: i32) -> Vec<usize> {
        let mut indices: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| match &brush.brush_type {
                BrushType::Circle | BrushType::Falloff => {
                    let dx = cell.x - cx;
                    let dy = cell.y - cy;
                    dx * dx + dy * dy <= brush.radius * brush.radius
                }
                BrushType::Square => {
                    (cell.x - cx).abs() <= brush.radius && (cell.y - cy).abs() <= brush.radius
                }
                BrushType::Custom(points) => points
                    .iter()
                    .any(|(x, y, weight)| *weight > 0 && cell.x == cx + *x && cell.y == cy + *y),
            })
            .map(|(index, _)| index)
            .collect();
        indices.sort();
        indices
    }

    fn brush_weight(&self, brush: &TerrainBrush, x: i32, y: i32, cx: i32, cy: i32) -> i32 {
        match &brush.brush_type {
            BrushType::Falloff => {
                let distance = (x - cx).abs() + (y - cy).abs();
                (brush.radius + 1 - distance).max(1)
            }
            BrushType::Custom(points) => points
                .iter()
                .find(|(px, py, _)| x == cx + *px && y == cy + *py)
                .map(|(_, _, weight)| (*weight).max(1))
                .unwrap_or(1),
            _ => 1,
        }
    }

    fn rebuild_hash(&mut self) {
        let cells = self
            .cells
            .iter()
            .map(|cell| {
                format!(
                    "{}:{}:{}:{}:{}",
                    cell.x,
                    cell.y,
                    cell.height,
                    cell.paint,
                    cell.metadata.join("|")
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        self.terrain_hash = stable_hash(&["terrain", &self.deterministic_seed.to_string(), &cells]);
    }
}

fn deterministic_noise(seed: u64, x: i32, y: i32) -> i32 {
    let hash = stable_hash(&[
        "terrain-noise",
        &seed.to_string(),
        &x.to_string(),
        &y.to_string(),
    ]);
    i32::from_str_radix(&hash[..6], 16).unwrap_or(0)
}

pub fn terrain_sculpting_equivalence() -> bool {
    let mut a = TerrainSurface::new(8, 8, 42);
    let mut b = TerrainSurface::new(8, 8, 42);
    let circle = TerrainBrush {
        brush_type: BrushType::Circle,
        radius: 2,
        strength: 3,
    };
    let square = TerrainBrush {
        brush_type: BrushType::Square,
        radius: 1,
        strength: 2,
    };
    let falloff = TerrainBrush {
        brush_type: BrushType::Falloff,
        radius: 2,
        strength: 1,
    };
    let custom = TerrainBrush {
        brush_type: BrushType::Custom(vec![(0, 0, 3), (1, 0, 2), (0, 1, 1)]),
        radius: 1,
        strength: 1,
    };
    for (tool, brush, x, y) in [
        (TerrainTool::Raise, &circle, 3, 3),
        (TerrainTool::Lower, &square, 2, 2),
        (TerrainTool::Smooth, &falloff, 3, 3),
        (TerrainTool::Flatten(5), &custom, 4, 4),
        (TerrainTool::Paint("stone"), &circle, 1, 1),
        (
            TerrainTool::Noise {
                seed: 7,
                amplitude: 4,
            },
            &square,
            5,
            5,
        ),
    ] {
        a.apply(tool.clone(), brush, x, y);
        b.apply(tool, brush, x, y);
    }
    a == b
}

pub fn region_painting_equivalence() -> bool {
    let mut a = TerrainSurface::new(8, 8, 7);
    let mut b = TerrainSurface::new(8, 8, 7);
    for kind in [
        RegionPaintKind::Region,
        RegionPaintKind::Partition,
        RegionPaintKind::Biome,
        RegionPaintKind::CivilizationTerritory,
        RegionPaintKind::ResourceZone,
        RegionPaintKind::SpawnZone,
    ] {
        a.paint_metadata(kind.clone(), "alpha", 4, 4, 2);
        b.paint_metadata(kind, "alpha", 4, 4, 2);
    }
    a == b
}
