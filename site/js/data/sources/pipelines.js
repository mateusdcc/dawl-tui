/* DTUI Source Code for Pipelines, Verification & Services */

export const DTUI_LOOP = `diagram loopEngineering "Loop engineering: one agent, one sequential context"
theme midnight
direction right

node input "INPUT\\nfeature request\\nacceptance criteria\\nrepo constraints" kind input
node phase "active task" kind phase

group cycle "single-agent task loop" kind repeat dashed

group planning "1 · plan" kind group in cycle
node planner "Planner agent\\nread request\\ninspect codebase\\nidentify risks" kind agent in planning
node plan "WORK PLAN\\nfiles • steps\\nverification contract" kind output in planning

group building "2 · implement" kind group in cycle
node developer "Developer agent\\nedit scoped files\\nkeep invariants\\nrecord decisions" kind agent in building
node tools "TOOLS\\nsearch\\nedit\\nbuild" kind shell in building
node patch "PATCH\\ncode + tests" kind output in building

group verifying "3 · verify" kind group in cycle
node reviewer "Verification agent\\nreview diff\\ncheck contract\\nchallenge assumptions" kind reviewer in verifying
node checks "EVIDENCE\\nunit tests\\nlint / build\\nUI screenshot" kind activity in verifying
node evidence "REPORT\\nlogs • citations" kind output in verifying
decision pass "Pass?" in verifying
node findings "FINDINGS\\nseverity • file\\nrequired fix" kind failure in verifying

node context "SHARED CONTEXT\\nplan • patch • logs\\nfindings accumulate" kind activity in cycle
node queue "WAITING\\nindependent tasks\\nblocked until this loop releases" kind shell in cycle

node checkpoint "Checkpoint\\ncommit artifact\\nadvance queue" kind success
node output "OUTPUT\\nverified patch\\nevidence report" kind output

edge receive input -> phase
edge start phase -> planner
edge plan_artifact planner -> plan
edge handoff plan -> developer
edge use_tools developer -> tools
edge produce_patch tools -> patch
edge patch_review patch -> reviewer
edge review_checks reviewer -> checks
edge checks_evidence checks -> evidence
edge evidence_gate evidence -> pass
edge rejected pass -> findings kind failure label "NO"
edge retry findings -> developer kind back label "fix + rerun"
edge accepted pass -> checkpoint kind success label "YES"
edge publish checkpoint -> output kind success
edge context_patch patch -> context kind muted
edge waiting input -> queue kind muted`;

export const DTUI_VERIFICATION = `diagram verificationOrchestrator "Verification orchestration: chained skills and clean context"
theme midnight
direction right

node feature "INPUT\\nfinalized feature\\nacceptance contract\\nrisk profile" kind input

group policyGroup "verification policy" kind group
node policy "RULE SET\\nrequired checks\\nseverity threshold\\nevidence schema" kind activity in policyGroup
node orchestrator "Orchestrator agent\\nselect skills\\nassign model tier\\nset stop conditions" kind agent in policyGroup
node dispatch "Run checks" kind fork in policyGroup

group skills "parallel verification skills" kind parallel

group embeddedLane "embedded skill · low-cost model" kind lane in skills
node embedded "Embedded runner\\nfast + deterministic" kind activity in embeddedLane
node embeddedWork "RUN\\nbuild • lint • unit tests\\nheadless UI screenshots" kind shell in embeddedLane
node embeddedReport "EVIDENCE\\nexit codes • logs\\nimage diffs" kind output in embeddedLane

group deepLane "deep-pass skill · high-tier model" kind lane in skills
node deep "Contextual reviewer\\nhigh-tier reasoning" kind reviewer in deepLane
node deepWork "REVIEW\\ncorrectness • security\\nnuance • false positives" kind activity in deepLane
node deepReport "FINDINGS\\nseverity • file\\nrationale + fix" kind output in deepLane

group opinionLane "second-opinion skill · clean high-tier session" kind lane in skills
node opinion "Second opinion\\nclean context (-p)" kind reviewer in opinionLane
node opinionWork "CHALLENGE\\nno prior conclusions\\ntest assumptions" kind activity in opinionLane
node opinionReport "VERDICT\\nindependent risks\\nconfidence" kind output in opinionLane

node barrier "UNIFIED REPORT\\nwait for all 3\\ndedupe + prioritize" kind join
decision verified "Verified?"
node output "OUTPUT\\nverified\\n+ evidence" kind success
node resolve "Worker agent\\nresolve findings\\nupdate tests\\nreturn evidence" kind failure

edge configure feature -> policy
edge select policy -> orchestrator
edge launch orchestrator -> dispatch
edge run_embedded dispatch -> embedded
edge run_deep dispatch -> deep
edge run_opinion dispatch -> opinion
edge embedded_execute embedded -> embeddedWork
edge embedded_emit embeddedWork -> embeddedReport
edge deep_execute deep -> deepWork
edge deep_emit deepWork -> deepReport
edge opinion_execute opinion -> opinionWork
edge opinion_emit opinionWork -> opinionReport
edge embedded_join embeddedReport -> barrier
edge deep_join deepReport -> barrier
edge opinion_join opinionReport -> barrier
edge assess barrier -> verified
edge pass verified -> output kind success label "YES"
edge fail verified -> resolve kind failure label "NO"
edge remediate resolve -> dispatch kind back label "fix + reverify"`;

export const DTUI_SIMPLE = `diagram pipeline "CI/CD Deployment Pipeline"
viewport 120x32
direction right
theme midnight

group ci "Build & Test" kind parallel at 2,3 size 45x18
node build "Compile" kind activity in ci at 6,7 size 14x5
node test "Unit Tests" kind activity in ci at 26,7 size 16x5
edge e1 build -> test

group cd "Deployment" kind repeat at 53,3 size 64x22
node staging "Staging Deploy" kind agent in cd at 58,7 size 18x5
decision verify "Healthy?" in cd at 83,7 size 11x5
node prod "Production" kind success at 100,16 size 16x5

edge e2 test -> staging
edge e3 staging -> verify
edge e4 verify -> prod kind success label "YES"
edge e5 verify -> staging kind back label "RETRY"

align horizontal build test staging verify`;

export const DTUI_RAG = `diagram rag "Autonomous RAG Workflow"
viewport 100x28
direction right
theme midnight

node query "User Query" kind input
node embed "Embedding Model" kind activity
node vdb "Vector DB Search" kind activity
node rerank "Reranker" kind reviewer
node llm "LLM Synthesizer" kind success

edge e1 query -> embed
edge e2 embed -> vdb
edge e3 vdb -> rerank
edge e4 rerank -> llm`;

export const DTUI_CANARY = `diagram canary "Canary Deployment Pipeline"
viewport 110x30
direction right
theme midnight

node build "Docker Build" kind activity
node canary "Canary Pods (10%)" kind agent
decision metrics "Error Rate < 0.1%?"
node full "Full Rollout (100%)" kind success
node abort "Rollback & Alert" kind failure

edge e1 build -> canary
edge e2 canary -> metrics
edge e3 metrics -> full kind success label "HEALTHY"
edge e4 metrics -> abort kind failure label "DEGRADED"`;

export const DTUI_SAGA = `diagram saga "Distributed Order Saga"
viewport 110x30
direction right
theme midnight

node order "Create Order" kind activity
node payment "Process Payment" kind activity
node inventory "Reserve Stock" kind activity
node ship "Schedule Delivery" kind success
node cancel "Compensate & Refund" kind failure

edge e1 order -> payment
edge e2 payment -> inventory
edge e3 inventory -> ship
edge e4 payment -> cancel kind failure label "FAILED"`;
