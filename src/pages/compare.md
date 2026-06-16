# How EverArcade Differs

EverArcade is not trying to replace every game platform. It is a different architecture for worlds that need continuity, portability, and independently checkable history.

| Model | Strength | Typical limitation | EverArcade difference |
| --- | --- | --- | --- |
| Traditional MMO | Deep authored worlds and large live operations. | World state, hosting, rules, and business decisions are usually inseparable from one publisher. | World packages, operator practice, checkpoints, and verification are designed as explicit architecture. |
| Roblox | Fast creation, social discovery, and accessible tooling. | Worlds live inside a platform-controlled runtime and distribution environment. | EverArcade emphasizes sovereign world packages and operator-hosted continuity. |
| Minecraft Servers | Modding culture and community-run servers. | Server history and rules are often tied to one host, mod stack, and backup practice. | Continuity, replay, restoration, and packaging become first-class concerns. |
| Steam Distribution | Powerful storefront and PC game distribution. | Distribution does not by itself make world history portable or verifiable. | EverArcade focuses on the runtime and continuity layer beneath distribution. |
| Blockchain Games | Public ownership experiments and open economic rails. | Gameplay can become secondary to token mechanics, and execution may not be a rich deterministic world runtime. | EverArcade separates world execution, verification, packaging, governance, and settlement boundaries. |
| EverArcade | Persistent worlds with portable packages, operator hosting, deterministic execution, replay verification, and continuity. | Still maturing; builders should check capability status before relying on a feature. | Built for worlds that should outlive one server, client, owner, or season. |

## Ownership model

EverArcade uses ownership language carefully. The goal is not just item ownership. It is world-level agency: rules, packages, history, recovery material, and operations should be understandable and portable enough for communities to preserve what they build.

## Architecture model

EverArcade separates the narrative layer from the implementation layer. The public website explains why persistent worlds matter; the docs explain contracts, runtime execution, replay, checkpointing, federation, packaging, and settlement boundaries.

## Learn more

- [Worlds](/worlds)
- [Developer Capability Matrix](/developers/capabilities)
- [Technical Overview](/developers/technical-overview)
