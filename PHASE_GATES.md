# Phase Gates & Success Criteria

**Last Updated:** May 4, 2026  
**Current Phase:** 1 (Setup)  
**Target Completion:** September 2026

---

## PHASE 1: Setup & Communication (Week 1)
**Status:** 🟡 IN PROGRESS  
**Owner:** @TODO (refactoring lead)  

### Objectives
1. Team reviews and understands architectural problems
2. Domain ownership clear and assigned
3. Feature freeze announced and enforced
4. Tracking infrastructure in place

### Success Criteria (All must pass to proceed)
- [ ] All team engineers have read ARCHITECTURE_AUDIT.md
- [ ] Team walkthrough completed (30 min)
- [ ] OWNERS_REFACTORING.md assigned to domain leads
- [ ] Tracking issue created on GitHub
- [ ] Feature branch `refactor/monorepo-restructure` created
- [ ] Build machine specs confirmed (8GB+ RAM, SSD)
- [ ] Written sign-off from engineering leads
- [ ] Weekly sync calendar blocked (Mon/Thu 30 min)
- [ ] Slack channel #refactoring created
- [ ] REFACTORING_PLAN.md shared and reviewed

### Blockers
*None yet*

### Notes
- Waiting for team to complete reading phase
- Meeting scheduled for Tuesday

---

## PHASE 2: Platform Extraction (Weeks 2-4)
**Status:** 🟨 QUEUED (starts Friday)  
**Owner:** @TODO (platform lead)  

### Objectives
1. Extract platform abstraction crate (`warp-platform`)
2. Move all OS-specific code to new crate
3. Consolidate system, menus, GPU, input modules
4. Reduce app/src from 80 → 73 modules

### Success Criteria (All must pass to proceed to Phase 3)
- [ ] `crates/platform/warp-platform/` exists with lib.rs
- [ ] Platform crate compiles standalone: `cargo build -p warp-platform`
- [ ] All platform modules moved (system, menus, gpu, input, antivirus, login_item)
- [ ] app/src module count < 75
- [ ] app/src still compiles: `cargo build --bin warp`
- [ ] No new circular dependencies introduced
- [ ] Incremental build time stable (±10% from baseline)
- [ ] Platform domain tests pass
- [ ] Code review approved by 1 other engineer

### Dependencies
*None (independent work)*

### Risks
- Hidden coupling in OS-specific code
- Build regressions when removing code from app/src
- Test coverage gaps in platform code

### Mitigations
- Daily `cargo build --bin warp` checks
- Create backup branch before starting
- Run full test suite daily

---

## PHASE 3: Terminal Analysis (Week 4)
**Status:** 🟨 QUEUED  
**Owner:** @TODO (terminal lead)  

### Objectives
1. Audit terminal code split across app/src and crates
2. Decide: consolidate vs. keep separate
3. Create consolidation plan

### Success Criteria (All must pass)
- [ ] Terminal audit document completed (TERMINAL_CONSOLIDATION_PLAN.md)
- [ ] Code categorization done (UI wrapper vs. business logic)
- [ ] Decision made: merge y/n and which files
- [ ] Architecture boundary documented
- [ ] Design decision reviewed and approved

### Dependencies
*Depends on Phase 2 completion*, but runs in parallel (different crate)

### Risks
- Unclear boundary between UI and logic
- Unexpected coupling with other domains

---

## PHASE 4: Directory & Crate Scaffolding (Weeks 5-6)
**Status:** 🟨 QUEUED  
**Owner:** @TODO (refactoring lead)  

### Objectives
1. Create all new crate directories and minimal Cargo.toml
2. Establish naming conventions and structure
3. Update workspace Cargo.toml

### Success Criteria (All must pass)
- [ ] All new crate directories created (75+ crates)
- [ ] Each has Cargo.toml + lib.rs/main.rs
- [ ] Root Cargo.toml updated with all members
- [ ] Workspace builds (may not link yet)
- [ ] CRATE_ORGANIZATION.md updated with new structure

### Dependencies
*Depends on Phase 2 complete* (platform extraction first)

---

## PHASE 5: Domain Extraction (Weeks 7-14)
**Status:** 🟨 QUEUED  
**Owner:** Domain leads (parallel teams)  

### Objectives
1. Move code from app/src into domain crates
2. Extract: AI, Cloud, Auth, Billing, Settings, Editor
3. Reduce app/src from ~73 → ~30 modules

### Success Criteria (All must pass)
- [ ] AI crate compiles: `cargo build -p warp-ai-core`
- [ ] Cloud crates compile: `cargo build -p warp-drive -p warp-auth -p warp-sync`
- [ ] Settings/Persistence crate compiles: `cargo build -p warp-persistence`
- [ ] ALL domain crates compile independently
- [ ] app/src module count ≤ 35
- [ ] Zero circular dependencies (checked by CI)
- [ ] Incremental build < 2 minutes
- [ ] Each domain crate has unit tests
- [ ] Code reviewed by domain owner + 1 other

### Dependencies
*Parallel work after Phase 4*

### Risks
- HIGH: Hidden coupling between domains
- Scope creep (extracting too much)
- Test suite fragmentation

### Mitigations
- Weekly architecture review
- CI enforces dependency graph DAG
- 2-3 engineers doing this in parallel (different domains)

---

## PHASE 6: Optimization (Weeks 15-16)
**Status:** 🟨 QUEUED  
**Owner:** @TODO (build systems lead)  

### Objectives
1. Configure CI for parallel builds
2. Add build profiles for fast dev
3. Optimize link-time dependencies

### Success Criteria (All must pass)
- [ ] CI builds in parallel (5+ independent jobs)
- [ ] Incremental build < 1 minute from cold
- [ ] Debug build < 2 minutes
- [ ] Release build < 5 minutes
- [ ] `--features fast_dev` works for rapid iteration
- [ ] Build telemetry tracked in CI

### Dependencies
*Depends on Phase 5 complete*

---

## PHASE 7: Finalization (Weeks 17-20)
**Status:** 🟨 QUEUED  
**Owner:** @TODO (refactoring lead)  

### Objectives
1. Comprehensive testing and validation
2. Merge to main
3. Team training on new structure
4. Documentation complete

### Success Criteria (All must pass to SHIP)
- [ ] Full integration test suite passes
- [ ] No performance regressions
- [ ] Incremental build < 60 seconds
- [ ] app/src ≤ 35 modules
- [ ] Zero circular dependencies
- [ ] All team members trained (knowledge share)
- [ ] CRATE_ORGANIZATION.md finalized
- [ ] OWNERS.md assigned and published
- [ ] PR reviewed and approved
- [ ] Merged to main

### Dependencies
*All previous phases*

---

## 📊 Checkpoint Matrix (Weekly)

| Week | Phase | Modules | Build Time | Cycles | Status |
|------|-------|---------|-----------|--------|--------|
| 1 | Setup | 80 | 5-10m | Many | 🟡 In Progress |
| 2-4 | Platform | <75 | 5-8m | Many | 🟨 Queued |
| 5-6 | Reorganize | <70 | 4-6m | Medium | 🟨 Queued |
| 7-14 | Domains | <35 | <2m | Few | 🟨 Queued |
| 15-16 | Optimize | <35 | <1m | 0 | 🟨 Queued |
| 17-20 | Finalize | <35 | <1m | 0 | 🟨 Queued |

---

## ⚠️ Go/No-Go Criteria

### Go to Phase 2:
✓ All Phase 1 success criteria met  
✓ Team written sign-off  
✓ Domain ownership assigned  
✓ Feature freeze active  

### Go to Phase 5:
✓ Platform extraction complete + tested  
✓ Build time stable  
✓ No unexpected circular deps  
✓ Crate structure in place  

### Go to Phase 7:
✓ All domain extraction complete  
✓ Each crate builds independently  
✓ Build time < 2 minutes  
✓ CI green for 3 consecutive days  

### Go to Main:
✓ All Phase 7 criteria met  
✓ Build time < 1 minute  
✓ Zero circular dependencies  
✓ Team sign-off for merge  

---

## 📞 Escalation

**Blocker?** → Post in #refactoring + mention @refactoring-lead  
**Design question?** → Friday architecture review (1 hour)  
**Merge conflict?** → Tag @refactoring-lead immediately  
**Build regression?** → Halt work, revert commit, debug + document  

---

**Last Review:** May 4, 2026  
**Next Review:** Monday May 6, 2026 (post-kickoff)
