/* Extensive Examples Data Library for dawl-tui */
import { DTUI_APPROVAL } from "./sources/approval.js";
import { DTUI_BARRIER, DTUI_DIAMOND } from "./sources/graphs.js";
import { DTUI_LOOP, DTUI_VERIFICATION, DTUI_SIMPLE, DTUI_RAG, DTUI_CANARY, DTUI_SAGA } from "./sources/pipelines.js";

export const EXAMPLES_DATA = [
  {
    id: "approval",
    title: "Multi-stage Approval Loop",
    category: "Approval Loops",
    description: "Complex human-in-the-loop workflow with security, compliance, and multi-tier approval nodes.",
    code: DTUI_APPROVAL,
    renderAscii: `+-------------------------------------------------------------+\n| SECURITY & AUDIT                                            |\n|  [ SecOps Review ] -------> < Approve Release? >            |\n|  [ Audit Logger ]                  |            |           |\n+------------------------------------+            |           |\n                                   PASS           REJECT      |\n                                     v            v           |\n                            [ Prod Deploy ]   [ Revert Patch ]|\n+-------------------------------------------------------------+`
  },
  {
    id: "graph-loop",
    title: "Loop Engineering Workflow",
    category: "Agent Workflows",
    description: "Sequential context loop connecting planner, developer, tool runner, and verification agent.",
    code: DTUI_LOOP,
    renderAscii: `[ Planner Agent ] ===> [ Developer Agent ] ===> [ Tool Runner ]\n       ^                                              |\n       |                                              v\n       +============== ( RETRY ) ============= [ Verifier ]`
  },
  {
    id: "graph-diamond",
    title: "Diamond Specialist Graph",
    category: "Agent Workflows",
    description: "Orchestrator creating parallel tasks for research, architecture, and test-design agents.",
    code: DTUI_DIAMOND,
    renderAscii: `                  +--> [ Research Agent ] ---+\n                  |                          |\n[ Orchestrator ] -+--> [ Architecture ] -----+-> [ Synthesis ]\n                  |                          |\n                  +--> [ Test Design ] ------+`
  },
  {
    id: "graph-barrier",
    title: "Fan-in Reviewer Barrier",
    category: "Security & Barriers",
    description: "Three parallel isolated reviewers (correctness, security, visual) with fail-closed barrier.",
    code: DTUI_BARRIER,
    renderAscii: `                 +--> [ Correctness Review ] --+\n                 |                             |\n[ Fork Candidate]+--> [ Security Audit ] ------+-> [ Barrier (3/3) ] -> < All Pass? >\n                 |                             |\n                 +--> [ Visual / UX Diff ] ----+`
  },
  {
    id: "graph-verification",
    title: "Verification Orchestrator",
    category: "Agent Workflows",
    description: "Chained verification skills with automated issue resolution and evidence collection.",
    code: DTUI_VERIFICATION,
    renderAscii: `[ Artifact Bundle ] -> [ Benchmark Spec ] -> [ Regression Test ] -> [ Certificate Emit ]`
  },
  {
    id: "simple",
    title: "Minimal CI/CD Pipeline",
    category: "CI/CD Pipelines",
    description: "Basic continuous delivery loop with compile, unit testing, staging, and production release.",
    code: DTUI_SIMPLE,
    renderAscii: `[ Compile ] ===> [ Unit Tests ] ===> [ Staging ] ===> [ Production ]`
  },
  {
    id: "agent-rag",
    title: "Autonomous RAG Workflow",
    category: "Agent Workflows",
    description: "Retrieval Augmented Generation agent with vector store lookup, reranking, and self-correction.",
    code: DTUI_RAG,
    renderAscii: `[ User Query ] -> [ Embedding ] -> [ Vector DB ] -> [ Reranker ] -> [ LLM Synth ]`
  },
  {
    id: "kubernetes-canary",
    title: "Canary Deployment & Rollback",
    category: "CI/CD Pipelines",
    description: "Progressive traffic split canary deployment with real-time error rate telemetry monitoring.",
    code: DTUI_CANARY,
    renderAscii: `[ Docker Build ] -> [ Canary (10%) ] -> < Metric Check? > --HEALTHY--> [ Full Rollout ]\n                                              |--DEGRADED---> [ Abort & Alert ]`
  },
  {
    id: "microservice-saga",
    title: "Distributed Saga Transaction",
    category: "Microservices",
    description: "Orchestrated saga for order processing with automatic compensating transactions on failure.",
    code: DTUI_SAGA,
    renderAscii: `[ Create Order ] -> [ Process Payment ] -> [ Reserve Stock ] -> [ Schedule Delivery ]\n                            |\n                            +--( PAYMENT_FAILED )---> [ Compensate & Refund ]`
  }
];
