| Field | Value |
| --- | --- |
| epoch hash | deterministic via canonical bincode over sorted epoch artifacts |
| receipt root | aggregated deterministic receipt hash |
| mutation root | aggregated deterministic mutation hash |
| stdout/event root | deterministic EventStream root |
| checkpoint root | deterministic checkpoint root |
| continuity proof | epoch continuity hash(prev,epoch,checkpoint) |
| replay equivalence | stable under canonical ordering |
| restoration equivalence | stable from checkpoint + compressed range |
