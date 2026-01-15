# Aquarium Simulator - Decision Log

## Purpose
Record significant technical and design decisions made during implementation.
Captures the "why" behind choices to inform future work.

---

## 2026-01-14: Initial Project Decisions (Pre-Implementation)

### Decision: Rust + ratatui
**Context**: Need to choose tech stack for terminal UI application.

**Options Considered**:
1. Rust + ratatui
2. Go + tview/bubbletea
3. Python + curses/textual
4. TypeScript + blessed/ink

**Chosen**: Rust + ratatui

**Rationale**:
- Performance: Rust's zero-cost abstractions suit real-time simulation
- Safety: Ownership system prevents common bugs in stateful applications
- ratatui: Mature, well-documented TUI library with good abstractions
- Learning opportunity: Deepens Rust experience

---

### Decision: Tier 1 Simulation Scope (Hunger + Health Only)
**Context**: Define initial simulation mechanics complexity.

**Options Considered**:
1. Full simulation (water quality, pH, temperature, disease)
2. Medium simulation (hunger, health, stress, happiness)
3. Light simulation (hunger, health only)

**Chosen**: Light simulation (Hunger + Health only)

**Rationale**:
- Start simple, expand later (YAGNI principle)
- Easier to balance and test
- Still differentiates from purely decorative solutions
- Can add complexity in Tier 2+ if desired

---

### Decision: Simple ASCII Visuals (`><>` style)
**Context**: Choose visual representation for fish.

**Options Considered**:
1. Multi-line ASCII art (detailed sprites)
2. Unicode fish characters
3. Simple 1-3 character ASCII (`><>`, `<><`)

**Chosen**: Simple ASCII (`><>` style)

**Rationale**:
- Wide terminal compatibility (no Unicode issues)
- Clear at a glance (readability over detail)
- Low rendering complexity
- Sufficient to convey direction and species

---

### Decision: Global Scatter Feeding (Not Targeted)
**Context**: Define feeding mechanic interaction model.

**Options Considered**:
1. Targeted feeding (click/select specific fish)
2. Zone-based feeding (food appears at cursor location)
3. Global scatter (one button, all fish eat)

**Chosen**: Global scatter

**Rationale**:
- Simplest to implement (no cursor tracking)
- Easiest to use (one key press)
- Fits terminal constraints (no mouse)
- Sufficient for Tier 1 mechanics

---

### Decision: Pause on Close (Not Background Daemon)
**Context**: Define simulation behavior when app is closed.

**Options Considered**:
1. Background daemon (simulation continues when closed)
2. Catch-up on load (calculate missed time on restart)
3. Pause (time stops when closed, resumes on open)

**Chosen**: Pause

**Rationale**:
- Simpler implementation (no daemon management)
- Predictable state (no surprises when reopening)
- Easier to debug (state frozen between sessions)
- User-friendly (fish won't die while away)

---

### Decision: Same-Species Breeding Only
**Context**: Define breeding compatibility rules.

**Options Considered**:
1. Cross-species breeding (create hybrids)
2. Breeding matrix (specific compatible pairs)
3. Same-species only

**Chosen**: Same-species only

**Rationale**:
- Realistic (no hybrid fish in nature for most aquarium species)
- Simpler logic (no compatibility matrix needed)
- Preserves species identity
- Easier to balance (no hybrid stat calculations)

---

### Decision: Instant Adult Offspring (No Growth Stages)
**Context**: Define breeding output complexity.

**Options Considered**:
1. Growth stages (fry -> juvenile -> adult with time)
2. Fry stage only (babies grow to adults over time)
3. Instant adults (babies spawn as full-grown)

**Chosen**: Instant adults

**Rationale**:
- Simplifies data model (no growth tracking)
- Reduces visual clutter (no tiny fry sprites)
- Faster gameplay loop (immediate results)
- Can add growth in Tier 2+ if desired

---

## Future Decision Template

### Decision: [Title]
**Context**: [What problem are we solving?]

**Options Considered**:
1. Option A
2. Option B
3. Option C

**Chosen**: [Selected option]

**Rationale**:
- [Reason 1]
- [Reason 2]
- [Trade-offs considered]

---
