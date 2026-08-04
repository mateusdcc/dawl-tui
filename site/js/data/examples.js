/* Extensive Examples Data Library for dawl-tui */
import { DTUI_APPROVAL } from "./sources/approval.js";
import { DTUI_BARRIER, DTUI_DIAMOND } from "./sources/graphs.js";
import { DTUI_LOOP, DTUI_VERIFICATION, DTUI_SIMPLE, DTUI_RAG, DTUI_CANARY, DTUI_SAGA } from "./sources/pipelines.js";

export const EXAMPLES_DATA = [
  {
    id: "graph-loop",
    title: "Loop Engineering Workflow",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Agent Workflows",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_LOOP,
    renderAscii: `[ Planner Agent ] ===> [ Developer Agent ] ===> [ Tool Runner ]\n       ^                                              |\n       |                                              v\n       +============== ( RETRY ) ============= [ Verifier ]`
  },
  {
    id: "graph-diamond",
    title: "Diamond Specialist Graph",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Agent Workflows",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_DIAMOND,
    renderAscii: `                  +--> [ Research Agent ] ---+\n                  |                          |\n[ Orchestrator ] -+--> [ Architecture ] -----+-> [ Synthesis ]\n                  |                          |\n                  +--> [ Test Design ] ------+`
  },
  {
    id: "graph-barrier",
    title: "Fan-in Reviewer Barrier",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Security & Barriers",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_BARRIER,
    renderAscii: `                 +--> [ Correctness Review ] --+\n                 |                             |\n[ Fork Candidate]+--> [ Security Audit ] ------+-> [ Barrier (3/3) ] -> < All Pass? >\n                 |                             |\n                 +--> [ Visual / UX Diff ] ----+`
  },
  {
    id: "graph-verification",
    title: "Verification Orchestrator",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Agent Workflows",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_VERIFICATION,
    renderAscii: `[ Artifact Bundle ] -> [ Benchmark Spec ] -> [ Regression Test ] -> [ Certificate Emit ]`
  },
  {
    id: "agent-rag",
    title: "Autonomous RAG Workflow",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Agent Workflows",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_RAG,
    renderAscii: `[ User Query ] -> [ Embedding ] -> [ Vector DB ] -> [ Reranker ] -> [ LLM Synth ]`
  },
  {
    id: "kubernetes-canary",
    title: "Canary Deployment & Rollback",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "CI/CD Pipelines",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_CANARY,
    renderAscii: `[ Docker Build ] -> [ Canary (10%) ] -> < Metric Check? > --HEALTHY--> [ Full Rollout ]\n                                              |--DEGRADED---> [ Abort & Alert ]`
  },
  {
    id: "microservice-saga",
    title: "Distributed Saga Transaction",
    category: "Pure Topology (Auto-Layout)",
    subCategory: "Microservices",
    isAutoLayout: true,
    description: "Sized entirely from topology and labels. Zero viewport, coordinates, or size syntax.",
    code: DTUI_SAGA,
    renderAscii: `[ Create Order ] -> [ Process Payment ] -> [ Reserve Stock ] -> [ Schedule Delivery ]\n                            |\n                            +--( PAYMENT_FAILED )---> [ Compensate & Refund ]`
  },
  {
    id: "approval",
    title: "Multi-stage Approval Loop",
    category: "Approval Loops",
    subCategory: "Manual Constraints",
    isAutoLayout: false,
    description: "Complex human-in-the-loop workflow with explicit port and rectangle layout placement.",
    code: DTUI_APPROVAL,
    renderAscii: `+-------------------------------------------------------------+\n| SECURITY & AUDIT                                            |\n|  [ SecOps Review ] -------> < Approve Release? >            |\n|  [ Audit Logger ]                  |            |           |\n+------------------------------------+            |           |\n                                   PASS           REJECT      |\n                                     v            v           |\n                            [ Prod Deploy ]   [ Revert Patch ]|\n+-------------------------------------------------------------+`
  },
  {
    id: "simple",
    title: "Minimal CI/CD Pipeline",
    category: "CI/CD Pipelines",
    subCategory: "Manual Constraints",
    isAutoLayout: false,
    description: "Basic continuous delivery loop with explicit alignment constraints and viewport bounding.",
    code: DTUI_SIMPLE,
    renderAscii: `[ Compile ] ===> [ Unit Tests ] ===> [ Staging ] ===> [ Production ]`
  }
];
