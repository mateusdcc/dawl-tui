/* DTUI Source Code for Approval Loop */
export const DTUI_APPROVAL = `diagram developIssuesUntilApproved "developIssuesUntilApproved: full agent flow"
viewport 202x72
theme midnight
direction right

node input "INPUT\\n\\nissues: 65, 66, …\\niterations: M (5)" at 1,25 size 19x8 kind input
node phase_issues "phase: issues" at 3,35 size 16x3 kind phase

group issues "parallel(\\"issues\\")" at 23,3 size 83x59 kind parallel

group issue65 "issue-65" at 25,5 size 78x18 kind lane in issues
group approval65 "developUntilApproved" at 43,6 size 57x16 kind group in issue65
group repeat65 "iteration 1…M" at 44,8 size 53x11 kind repeat in approval65 dashed
node wt65 "worktree: issue 65\\nisolated branch" at 26,9 size 17x6 kind shell in issue65
node dev65 "Developer ag\\nimplement\\nfix findings" at 46,9 size 15x5 kind agent in repeat65
node rev65 "Reviewer ag\\npass / notes" at 64,9 size 14x5 kind reviewer in repeat65
decision pass65 "pass?" at 82,9 size 9x5 in repeat65
node fail65 "failed review" at 79,15 size 15x3 kind failure in repeat65
text note65 "new pair each retry" at 56,19 kind dim

group issue66 "issue-66" at 25,24 size 78x18 kind lane in issues
group approval66 "developUntilApproved" at 43,25 size 57x16 kind group in issue66
group repeat66 "iteration 1…M" at 44,27 size 53x11 kind repeat in approval66 dashed
node wt66 "worktree: issue 66\\nisolated branch" at 26,28 size 17x6 kind shell in issue66
node dev66 "Developer ag\\nimplement\\nfix findings" at 46,28 size 15x5 kind agent in repeat66
node rev66 "Reviewer ag\\npass / notes" at 64,28 size 14x5 kind reviewer in repeat66
decision pass66 "pass?" at 82,28 size 9x5 in repeat66
node fail66 "failed review" at 79,34 size 15x3 kind failure in repeat66
text note66 "new pair each retry" at 56,38 kind dim

node issueResults "issueResults" at 109,29 size 14x5 kind join
node phase_merge "merge phase" at 125,29 size 13x3 kind phase

group mergeApproval "developUntilApproved\\non main" at 141,5 size 25x57 kind group
group mergeRepeat "iteration 1…M" at 142,8 size 23x37 kind repeat in mergeApproval dashed
node mergeDev "Merge Dev\\nagent" at 145,11 size 15x5 kind agent in mergeRepeat
node mergeRev "Merge Review\\nagent" at 145,20 size 15x5 kind reviewer in mergeRepeat
decision mergePass "pass?" at 148,28 size 9x5 in mergeRepeat

node cleanup "shell cleanup\\nremove merged\\nworktrees" at 169,24 size 15x13 kind shell
node output "OUTPUT\\n\\nissues • results\\nmerge • summary" at 183,53 size 18x6 kind output`;
