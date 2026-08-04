/* DTUI Source Code for Graph Engineering Gallery */

export const DTUI_BARRIER = `diagram fanInBarrier "Fan-in barrier: three independent release reviews"
theme midnight
direction right

node candidate "CANDIDATE\\nfeature patch\\ntests • screenshots\\nacceptance criteria" kind input
node split "Evaluate" kind fork

group reviewers "parallel review nodes · isolated perspectives" kind parallel

group correctnessLane "correctness contract" kind lane in reviewers
node correctness "Correctness agent\\nhigh-tier review" kind reviewer in correctnessLane
node correctnessChecks "CHECKS\\nrequirements • invariants\\nedge cases • regression" kind activity in correctnessLane
node correctnessReport "VERDICT\\nPASS or defects\\ntest evidence" kind output in correctnessLane

group securityLane "security contract" kind lane in reviewers
node security "Security agent\\nhigh-tier review" kind reviewer in securityLane
node securityChecks "CHECKS\\nauth • secrets • input\\ndeps • threat paths" kind activity in securityLane
node securityReport "VERDICT\\nPASS or findings\\nattack narrative" kind output in securityLane

group visualLane "visual / UX contract" kind lane in reviewers
node visual "Visual agent\\nheadless browser" kind reviewer in visualLane
node visualChecks "CHECKS\\nscreenshots • responsive\\na11y • baseline diff" kind activity in visualLane
node visualReport "VERDICT\\nPASS or defects\\nimage evidence" kind output in visualLane

node barrier "BARRIER\\nwait for 3/3\\nmerge findings" kind join
decision release "Release?"
node publish "PUBLISH\\nrelease bundle\\n+ evidence" kind success
node repair "Repair worker\\nassign owners\\nresolve findings\\nattach evidence" kind failure

edge fan candidate -> split
edge review_correctness split -> correctness
edge review_security split -> security
edge review_visual split -> visual
edge correctness_run correctness -> correctnessChecks
edge correctness_emit correctnessChecks -> correctnessReport
edge security_run security -> securityChecks
edge security_emit securityChecks -> securityReport
edge visual_run visual -> visualChecks
edge visual_emit visualChecks -> visualReport
edge correctness_join correctnessReport -> barrier
edge security_join securityReport -> barrier
edge visual_join visualReport -> barrier
edge decide barrier -> release
edge approved release -> publish kind success label "3/3 PASS"
edge rejected release -> repair kind failure label "ANY FAIL"
edge recheck repair -> split kind back label "targeted rerun"`;

export const DTUI_DIAMOND = `diagram diamondGraph "Diamond graph: parallel specialists, one synthesis"
theme midnight
direction right

node input "INPUT\\ncomplex goal\\nconstraints\\nsuccess criteria" kind input

group orchestration "orchestration" kind group
node orchestrator "Orchestrator\\ndecompose goal\\nset contracts\\nchoose model tier" kind agent in orchestration
node contracts "TASK CONTRACTS\\ninput • scope\\noutput schema\\ndone condition" kind activity in orchestration
node dispatch "Dispatch" kind fork in orchestration

group workers "parallel isolated context windows" kind parallel

group researchLane "research contract · low-cost model" kind lane in workers
node research "Research agent\\nsearch sources\\nvalidate facts" kind agent in researchLane
node researchTask "TASKS\\nAPIs • constraints\\nprior art • citations" kind activity in researchLane
node evidence "EVIDENCE PACK\\nsource links\\nknown unknowns" kind output in researchLane

group architectureLane "architecture contract · high-tier model" kind lane in workers
node architect "Architecture agent\\nreason over tradeoffs\\nresolve constraints" kind agent in architectureLane
node architectureTask "TASKS\\nboundaries • data flow\\nfailure modes • cost" kind activity in architectureLane
node design "DESIGN ARTIFACT\\ncomponents\\ndecision record" kind output in architectureLane

group testingLane "test contract · low-cost model" kind lane in workers
node tester "Test-design agent\\nchallenge plan\\nfind edge cases" kind agent in testingLane
node testingTask "TASKS\\ninvariants • fixtures\\nnegative paths • oracles" kind activity in testingLane
node testPlan "TEST PLAN\\ncases + expected\\nevidence" kind output in testingLane

node bundle "Context pack\\nall 3 artifacts\\nsource-linked" kind join
node synthesizer "Synthesis agent\\nhigh-tier model\\nresolve conflicts\\nproduce final plan" kind reviewer
decision complete "Complete?"
node output "OUTPUT\\nfinal plan\\nrisks\\n+ tests" kind output

edge receive input -> orchestrator
edge define orchestrator -> contracts
edge dispatch_contracts contracts -> dispatch
edge to_research dispatch -> research
edge to_architecture dispatch -> architect
edge to_testing dispatch -> tester
edge research_work research -> researchTask
edge research_output researchTask -> evidence
edge architecture_work architect -> architectureTask
edge architecture_output architectureTask -> design
edge testing_work tester -> testingTask
edge testing_output testingTask -> testPlan
edge collect_evidence evidence -> bundle
edge collect_design design -> bundle
edge collect_tests testPlan -> bundle
edge synthesize bundle -> synthesizer
edge assess synthesizer -> complete
edge ship complete -> output kind success label "YES"
edge retry complete -> dispatch kind back label "missing evidence"`;
