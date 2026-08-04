/* Extensive Examples Data Library for dawl-tui */
export const EXAMPLES_DATA = [
  {
    id: "approval",
    title: "Multi-stage Approval Loop",
    category: "Approval Loops",
    description: "Complex human-in-the-loop workflow with security, compliance, and multi-tier approval nodes.",
    code: `diagram approval "Multi-stage Approval Loop"\nviewport 120x32\ndirection right\ntheme midnight\n\ngroup security "Security & Audit" kind parallel\nnode sec "SecOps Review" kind reviewer in security\nnode audit "Audit Logger" kind activity in security\n\ndecision gate "Approve Release?"\nnode release "Prod Deploy" kind success\nnode rollback "Revert Patch" kind failure\n\nedge e1 sec -> gate\nedge e2 gate -> release kind success label "PASS"\nedge e3 gate -> rollback kind failure label "REJECT"`,
    renderAscii: `+-------------------------------------------------------------+\n| SECURITY & AUDIT                                            |\n|  [ SecOps Review ] -------> < Approve Release? >            |\n|  [ Audit Logger ]                  |            |           |\n+------------------------------------+            |           |\n                                   PASS           REJECT      |\n                                     v            v           |\n                            [ Prod Deploy ]   [ Revert Patch ]|\n+-------------------------------------------------------------+`
  },
  {
    id: "graph-loop",
    title: "Loop Engineering Workflow",
    category: "Agent Workflows",
    description: "Sequential context loop connecting planner, developer, tool runner, and verification agent.",
    code: `diagram graphLoop "Loop Engineering Workflow"\ntheme midnight\ndirection right\n\nnode plan "Planner Agent" kind agent\nnode dev "Developer Agent" kind agent\nnode tool "Tool Runner" kind activity\nnode verify "Verifier" kind reviewer\n\nedge e1 plan -> dev\nedge e2 dev -> tool\nedge e3 tool -> verify\nedge e4 verify -> plan kind back label "RETRY"`,
    renderAscii: `[ Planner Agent ] ===> [ Developer Agent ] ===> [ Tool Runner ]\n       ^                                              |\n       |                                              v\n       +============== ( RETRY ) ============= [ Verifier ]`
  },
  {
    id: "graph-diamond",
    title: "Diamond Specialist Graph",
    category: "Agent Workflows",
    description: "Orchestrator creating parallel tasks for research, architecture, and test-design agents.",
    code: `diagram graphDiamond "Diamond Specialist Graph"\ntheme midnight\ndirection right\n\nnode orch "Orchestrator" kind agent\nnode res "Research Agent" kind agent\nnode arch "Architecture" kind agent\nnode test "Test Design" kind agent\nnode synth "Synthesis Engine" kind success\n\nedge e1 orch -> res\nedge e2 orch -> arch\nedge e3 orch -> test\nedge e4 res -> synth\nedge e5 arch -> synth\nedge e6 test -> synth`,
    renderAscii: `                  +--> [ Research Agent ] ---+\n                  |                          |\n[ Orchestrator ] -+--> [ Architecture ] -----+-> [ Synthesis ]\n                  |                          |\n                  +--> [ Test Design ] ------+`
  },
  {
    id: "graph-barrier",
    title: "Fan-in Reviewer Barrier",
    category: "Security & Barriers",
    description: "Three parallel isolated reviewers (correctness, security, visual) with fail-closed barrier.",
    code: `diagram fanInBarrier "Fan-in Barrier"\ntheme midnight\ndirection right\n\nnode split "Fork Candidate" kind fork\nnode c1 "Correctness Review" kind reviewer\nnode c2 "Security Audit" kind reviewer\nnode c3 "Visual / UX Diff" kind reviewer\nnode join "Barrier Join (3/3)" kind join\ndecision pass "All Pass?"\n\nedge e1 split -> c1\nedge e2 split -> c2\nedge e3 split -> c3\nedge e4 c1 -> join\nedge e5 c2 -> join\nedge e6 c3 -> join\nedge e7 join -> pass`,
    renderAscii: `                 +--> [ Correctness Review ] --+\n                 |                             |\n[ Fork Candidate]+--> [ Security Audit ] ------+-> [ Barrier (3/3) ] -> < All Pass? >\n                 |                             |\n                 +--> [ Visual / UX Diff ] ----+`
  },
  {
    id: "graph-verification",
    title: "Verification Orchestrator",
    category: "Agent Workflows",
    description: "Chained verification skills with automated issue resolution and evidence collection.",
    code: `diagram verification "Verification Pipeline"\ntheme midnight\n\nnode input "Artifact Bundle" kind input\nnode bench "Benchmark Spec" kind activity\nnode regression "Regression Test" kind reviewer\nnode cert "Certificate Emit" kind success\n\nedge e1 input -> bench\nedge e2 bench -> regression\nedge e3 regression -> cert kind success label "VALIDATED"`,
    renderAscii: `[ Artifact Bundle ] -> [ Benchmark Spec ] -> [ Regression Test ] -> [ Certificate Emit ]`
  },
  {
    id: "simple",
    title: "Minimal CI/CD Pipeline",
    category: "CI/CD Pipelines",
    description: "Basic continuous delivery loop with compile, unit testing, staging, and production release.",
    code: `diagram pipeline "CI/CD Pipeline"\nviewport 120x32\ndirection right\n\nnode build "Compile" kind activity\nnode test "Unit Tests" kind activity\nnode stage "Staging" kind agent\nnode prod "Production" kind success\n\nedge e1 build -> test\nedge e2 test -> stage\nedge e3 stage -> prod`,
    renderAscii: `[ Compile ] ===> [ Unit Tests ] ===> [ Staging ] ===> [ Production ]`
  },
  {
    id: "agent-rag",
    title: "Autonomous RAG Workflow",
    category: "Agent Workflows",
    description: "Retrieval Augmented Generation agent with vector store lookup, reranking, and self-correction.",
    code: `diagram rag "Autonomous RAG"\ntheme midnight\n\nnode query "User Query" kind input\nnode embed "Embedding Model" kind activity\nnode vdb "Vector DB Search" kind activity\nnode rerank "Reranker" kind reviewer\nnode llm "LLM Synthesizer" kind success\n\nedge e1 query -> embed\nedge e2 embed -> vdb\nedge e3 vdb -> rerank\nedge e4 rerank -> llm`,
    renderAscii: `[ User Query ] -> [ Embedding ] -> [ Vector DB ] -> [ Reranker ] -> [ LLM Synth ]`
  },
  {
    id: "kubernetes-canary",
    title: "Canary Deployment & Rollback",
    category: "CI/CD Pipelines",
    description: "Progressive traffic split canary deployment with real-time error rate telemetry monitoring.",
    code: `diagram canary "Canary Deployment"\ntheme midnight\n\nnode build "Docker Build" kind activity\nnode canary "Canary Pods (10%)" kind agent\ndecision metrics "Error Rate < 0.1%?"\nnode full "Full Rollout (100%)" kind success\nnode abort "Rollback & Alert" kind failure\n\nedge e1 build -> canary\nedge e2 canary -> metrics\nedge e3 metrics -> full kind success label "HEALTHY"\nedge e4 metrics -> abort kind failure label "DEGRADED"`,
    renderAscii: `[ Docker Build ] -> [ Canary (10%) ] -> < Metric Check? > --HEALTHY--> [ Full Rollout ]\n                                              |--DEGRADED---> [ Abort & Alert ]`
  },
  {
    id: "microservice-saga",
    title: "Distributed Saga Transaction",
    category: "Microservices",
    description: "Orchestrated saga for order processing with automatic compensating transactions on failure.",
    code: `diagram saga "Order Saga"\ntheme midnight\n\nnode order "Create Order" kind activity\nnode payment "Process Payment" kind activity\nnode inventory "Reserve Stock" kind activity\nnode ship "Schedule Delivery" kind success\nnode cancel "Compensate & Refund" kind failure\n\nedge e1 order -> payment\nedge e2 payment -> inventory\nedge e3 inventory -> ship\nedge e4 payment -> cancel kind failure label "PAYMENT_FAILED"`,
    renderAscii: `[ Create Order ] -> [ Process Payment ] -> [ Reserve Stock ] -> [ Schedule Delivery ]\n                            |\n                            +--( PAYMENT_FAILED )---> [ Compensate & Refund ]`
  }
];
