# Monorepo Refactoring Implementation Plan

**Status:** Week 1 Setup - ACTIVE  
**Created:** May 4, 2026  
**Target Completion:** September 2026 (16-20 weeks)  
**Team:** 2-3 engineers  

---

## 🎯 Project Objectives

1. **Reduce app/src from 80 modules to 30 modules** by extracting into coordinated domain crates
2. **Improve incremental build time from 5-10 min to <60 seconds** through modularity
3. **Establish clear architecture boundaries** with zero circular dependencies
4. **Create onboarding-friendly structure** for new developers

**Success Criteria:**
- [ ] Incremental build time < 60 seconds (from main)
- [ ] Zero circular dependencies (verified by CI)
- [ ] app/src crate has ≤ 35 modules
- [ ] Each domain crate can build independently
- [ ] OWNERS file clearly assigns domain responsibility
- [ ] Team comfortable explaining new structure to newcomers

---

## 📋 Phase Breakdown

| Phase | Weeks | What | Team Effort |
|-------|-------|------|------------|
| **Phase 1: Setup** | 1 | Planning, docs, tracking | 1 eng-week |
| **Phase 2: Platform** | 2-4 (3 weeks) | OS abstraction extraction | 6 eng-weeks |
| **Phase 3: Terminal** | 5-6 (2 weeks) | Terminal consolidation | 4 eng-weeks |
| **Phase 4: Reorganize** | 7-8 (2 weeks) | Crate structure & naming | 4 eng-weeks |
| **Phase 5: Domains** | 9-14 (6 weeks) | Extract AI, Cloud, Auth, Billing | 12 eng-weeks |
| **Phase 6: Optimize** | 15-16 (2 weeks) | CI parallelization, build profiles | 4 eng-weeks |
| **Phase 7: Finalize** | 17-20 (4 weeks) | Testing, docs, team training | 8 eng-weeks |

**Total:** 16-20 weeks, 2-3 engineers concurrent

---

## 📅 WEEK 1 CHECKLIST (This Week)

### Monday (Today)
- [x] Team circulates audit documents (ARCHITECTURE_AUDIT.md)
- [x] Schedule 30-min team walkthrough
- [x] Create tracking issue on GitHub
- [ ] Request feature freeze (branch protection)

### Tuesday
- [ ] **Live:** Team discussion on timeline/risks
- [ ] **Live:** Identify domain owners (see OWNERS file template below)
- [ ] **Live:** Confirm build machine specs
- [ ] **Live:** Review OWNERS file draft

### Wednesday
- [ ] Create feature branch: `refactor/monorepo-restructure`
- [ ] Push initial planning docs to branch
- [ ] Update root Cargo.toml with refactoring notes
- [ ] Share REFACTORING_PLAN.md with team

### Thursday-Friday
- [ ] Get written engineering sign-off (team comments on plan)
- [ ] Schedule recurring Monday/Thursday 30-min syncs
- [ ] Prepare build machine (update dependencies)
- [ ] **Friday EOD:** Begin Phase 2 on Monday

---

## 🏗️ Domain Ownership (OWNERS Draft)

**AI Domain:**
- Owner: @TODO
- Files: `app/src/ai/`, `crates/ai/`
- Contact: Slack #backend

**Terminal Domain:**
- Owner: @TODO
- Files: `app/src/terminal/`, `crates/warp_terminal/`
- Contact: Slack #backend

**Cloud/Sync Domain:**
- Owner: @TODO
- Files: `app/src/sync/`, `app/src/auth/`, `app/src/billing/`
- Contact: Slack #backend

**UI Domain:**
- Owner: @TODO
- Files: `app/src/ui/`, `crates/warpui/`
- Contact: Slack #frontend

**Platform Domain:**
- Owner: @TODO
- Files: `app/src/platform/`, `app/src/system/`, `crates/platform/` (new)
- Contact: Slack #backend

**Settings/Persistence Domain:**
- Owner: @TODO
- Files: `app/src/settings/`, `crates/settings/`
- Contact: Slack #backend

---

## 🔧 Git Strategy

**Feature Branch:** `refactor/monorepo-restructure`
- Single branch for entire 16-week refactoring
- Daily commits with clear messages: `refactor: {phase}/{domain} - {description}`
- Merge to main when complete (Phase 7)

**Backup Branch:** `backup-pre-refactor`
- Created before starting Week 2
- Never touched after that (safety net)

**CI Enforcement:** 
- No merges to main without passing architecture checks
- Build time must never exceed baseline + 10%

---

## 📊 Weekly Sync Format

**Every Monday & Thursday, 30 minutes**

1. **Blockers** (5 min): Any stuck work?
2. **Progress** (10 min): % of modules moved, build time trend
3. **Risks** (10 min): Anything slipping? Hidden dependencies?
4. **Next Week** (5 min): Confirm tasks for next phase

---

## 🚨 Risk Mitigation

| Risk | Likelihood | Mitigation |
|------|-----------|-----------|
| **Hidden coupling** | HIGH | Surface early (Week 2), document in ISSUE |
| **Build regression** | MEDIUM | CI prevents merge if build time > baseline |
| **Merge conflicts** | MEDIUM | Coordinate across 2-3 engineers |
| **Scope creep** | MEDIUM | Strict phase gates; no new features during refactor |
| **Team confusion** | LOW | Weekly docs + Slack channel #refactoring |

---

## 📞 Communication Channels

- **Slack Channel:** #refactoring (daily updates, questions)
- **Weekly Meeting:** Mon 2pm PT (sync + planning)
- **Weekly Ad-hoc:** Thu 2pm PT (blockers + risks)
- **GitHub Issues:** Tracking blockers, questions, design decisions
- **REFACTORING_PLAN.md:** Source of truth (updated weekly)

---

## ✅ Phase Gates

**After Week 1:** Team sign-off + OWNERS assigned → Proceed to Phase 2  
**After Week 4:** Platform extraction complete + app/src < 75 modules → Proceed to Phase 3  
**After Week 8:** Terminal + Reorganization done + build time improving → Continue Phases 5-6  
**After Week 14:** All domain extraction complete + zero cycles → Optimization phase  
**After Week 16:** CI passing, build clean → Finalization and team training  

---

## 📈 Success Metrics (Weekly Tracking)

| Metric | Week 1 | Week 4 | Week 8 | Week 16 | Target |
|--------|--------|--------|--------|---------|--------|
| **app/src modules** | 80 | <75 | <60 | <35 | <35 ✓ |
| **Build time (incremental)** | 5-10m | 4-6m | 2-3m | <1m | <1m ✓ |
| **Circular deps** | Many | Medium | Few | 0 | 0 ✓ |
| **Crate count** | 45 | 48 | 60 | 75+ | 75+ ✓ |
| **CI passes** | 100% | 100% | 100% | 100% | 100% ✓ |

---

## 📝 Deliverables by Phase

- **Phase 1:** ✓ REFACTORING_PLAN.md, OWNERS file, tracking, sign-off
- **Phase 2-3:** warp-platform crate, terminal consolidation
- **Phase 4:** New crate structure, renaming
- **Phase 5:** 6 new domain crates (AI, Cloud, Auth, Billing, etc)
- **Phase 6:** CI parallelization, build profiles
- **Phase 7:** Final migration, CRATE_ORGANIZATION.md, team training

---

## 🚀 Next Steps (This Week)

1. **Today:** Distribute to team, schedule kickoff
2. **Tomorrow:** Team discussion + OWNERS assignment
3. **Wednesday:** Create feature branch + setup
4. **Friday:** Team sign-off → Ready for Phase 2

---

## 📞 Questions?

For detailed info, see:
- **Strategy:** ARCHITECTURE_AUDIT.md (Phase 1-2 section)
- **Week-by-week tasks:** REFACTORING_TACTICS.md
- **Code organization:** CRATE_ORGANIZATION.md

---

**Status: READY FOR TEAM REVIEW**
