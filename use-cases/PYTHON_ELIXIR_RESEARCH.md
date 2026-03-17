# Research: Python to Elixir Migrations in AI/Agent Orchestration

**Date:** 2026-03-14
**Status:** Research complete

---

## The Narrative: Why Is This Trend Happening?

The buzz comes from a simple observation: **the actor model that Erlang introduced in 1986 is the agent model that AI is rediscovering in 2026.** Every pattern the Python AI ecosystem is building -- isolated state, message passing, supervision hierarchies, fault recovery -- already exists in the BEAM virtual machine.

Python dominates model training and inference, but it's a poor fit for **orchestrating** many concurrent agents. The GIL, memory bloat, lack of process isolation, and absence of supervision trees mean Python agent frameworks end up reimplementing (badly) what OTP provides natively.

The catalyst moment was **OpenAI releasing Symphony in March 2026** -- an autonomous agent orchestration framework built in Elixir/OTP. OpenAI explicitly cited BEAM's ability to supervise long-running processes and hot-reload code without stopping active agents. When OpenAI validates Elixir for this use case, people pay attention.

### Key arguments for Elixir over Python in agent orchestration:

| Problem | Python Approach | BEAM/OTP Approach |
|---------|----------------|-------------------|
| Agent isolation | Async tasks sharing memory | Erlang processes (~2KB each, isolated GC) |
| Communication | Custom message layers | Built-in message passing |
| Fault recovery | Application-level try/except | Supervisor trees (native since 1998) |
| Concurrency | GIL-limited threads | Preemptive scheduling, millions of processes |
| Hot reload | Restart the service | Hot code reload without dropping active agents |

Source: [Your Agent Framework Is Just a Bad Clone of Elixir](https://georgeguimaraes.com/your-agent-orchestrator-is-just-a-bad-clone-of-elixir/) by George Guimaraes

---

## Concrete Examples Found

### 1. OpenAI Symphony (March 2026) -- The Big Signal

OpenAI open-sourced [Symphony](https://github.com/openai/symphony), an Elixir/OTP service that watches Linear boards, dispatches Codex coding agents, manages multi-turn execution in isolated workspaces, handles CI testing, and delivers PRs. This is the highest-profile validation of Elixir for agent orchestration.

- [GitHub repo](https://github.com/openai/symphony)
- [Elixir Forum discussion](https://elixirforum.com/t/openai-released-a-library-that-uses-elixir-to-orchestrate-ai-agents/74520)
- [MarkTechPost writeup](https://www.marktechpost.com/2026/03/05/openai-releases-symphony-an-open-source-agentic-framework-for-orchestrating-autonomous-ai-agents-through-structured-scalable-implementation-runs/)

**Note:** Symphony is not a Python-to-Elixir migration. It was built in Elixir from the start.

### 2. Mr. Popov's SaaS Migration -- The One Real Migration Story

The most concrete Python-to-Elixir migration documented is a SaaS platform in accounting/document processing:

- **Before:** FastAPI (Python), React 18, Camunda/Zeebe workflow engine, MySQL, Redis, Elasticsearch, 3-person team
- **After:** Phoenix 1.8 monolith with LiveView, 1 developer
- **Results:** 63% less code (68,850 -> 25,185 LOC), 75% fewer files, 92.9% less frontend code, 9 technologies eliminated
- **Timeline:** ~3 weeks for full rewrite (clean-slate, not incremental)

Source: [Python React to Elixir Phoenix Migration Breakdown](https://mrpopov.com/posts/python-react-to-elixir-phoenix-migration-breakdown/)

**Important caveat:** This was a general web app migration, not specifically AI/agent related. The company is under NDA.

### 3. Folder IT -- Elixir-Native AI Agents (Not a Migration)

Folder IT built a production AI agent system using LangChain (Elixir), Reactor, and MCP tools. Multiple agents running in production processing data and generating insights. Published October 2025.

Source: [How Elixir Helped Us Integrate AI Agents at Lightning Speed](https://folderit.net/how-elixir-helped-us-integrate-ai-agents-at-lightning-speed/)

**Note:** Not a migration from Python. They were already an Elixir shop.

### 4. V7 -- Elixir Orchestrating Python ML Nodes

V7, a computer vision platform, uses Elixir/Phoenix/Cowboy to handle data and orchestrate Python nodes for ML tasks. This is the **hybrid pattern** -- Elixir for orchestration, Python for ML.

Source: [Elixir Production Adoption: Top Companies](https://www.curiosum.com/blog/adoption-of-elixir-by-top-companies)

### 5. Stuart (Delivery Company) -- Python Embedded in Elixir

Stuart runs Python within their Elixir application for specific ML workloads rather than migrating.

Source: [How We Use Python Within Elixir](https://medium.com/stuart-engineering/how-we-use-python-within-elixir-486eb4d266f9)

---

## The Elixir ML/AI Ecosystem

The ecosystem is real but young:

- **[Nx](https://github.com/elixir-nx)** -- Numerical computing library with CPU/GPU compilation via Google XLA and Libtorch. 18 projects in the Nx org.
- **[Bumblebee](https://github.com/elixir-nx/bumblebee)** -- Pre-trained model integration with HuggingFace. Supports Llama, Mistral, Stable Diffusion, etc.
- **[Pythonx](https://github.com/livebook-dev/pythonx)** -- Embed Python interpreter directly in Elixir (same OS process, shared memory). Dashbit's "year of interoperability" project for 2025.
- **[Livebook](https://livebook.dev)** -- Computational notebook (Elixir's Jupyter). Now supports Python cells with autocompletion.
- **[Jido](https://github.com/agentjido/jido)** -- Autonomous agent framework for Elixir with OTP runtime.
- **[SwarmEx](https://github.com/nshkrdotcom/synapse)** -- Agent orchestration with telemetry.
- **[Agens](https://github.com/jessedrelick/agens)** -- Multi-agent workflows using OTP components.

Dashbit (Jose Valim's company) explicitly called 2025 the "year of interoperability," focusing on zero-copy Apache Arrow data transfer between Elixir and Python, and distributed Python execution via Erlang distribution.

Sources:
- [Embedding Python in Elixir, it's Fine](https://dashbit.co/blog/running-python-in-elixir-its-fine)
- [Dashbit Plans for 2025](https://dashbit.co/blog/dashbit-plans-2025)
- [Distributed Python Dataframes with Livebook](https://dashbit.co/blog/distributed-python-livebook)

---

## Assessment: Real Migration Wave or Hype?

**Mostly hype, but with a real kernel.**

### What's real:
- The technical argument is sound. BEAM/OTP genuinely solves agent orchestration problems that Python fights against.
- OpenAI choosing Elixir for Symphony is a legitimate signal, not marketing.
- The Pythonx/interop story means you don't have to abandon Python's ML ecosystem.
- A handful of companies (V7, Stuart, Folder IT) are running Elixir+AI in production.
- The Nx/Bumblebee ecosystem is functional and growing.

### What's hype:
- **I found exactly ONE documented Python-to-Elixir migration** (Mr. Popov's SaaS app), and it wasn't AI-related.
- Most "Elixir for AI" content is blog posts arguing why you *should* migrate, not reports from teams that *did*.
- The George Guimaraes article (the most-cited piece) contains no actual migration case studies.
- Most real-world adoption is **hybrid** (Elixir orchestrating Python) or **greenfield** (new Elixir projects), not migrations.
- The Elixir community is small. Finding Elixir developers is already hard; finding ones who also know AI/ML is harder.
- Python's ecosystem advantage for model development is enormous and not going away.

### The honest picture:
The emerging pattern is **not** "rewrite your Python AI code in Elixir." It's:
1. **Keep Python for model inference and ML libraries**
2. **Use Elixir for the orchestration layer** (supervisor trees, message passing, fault tolerance)
3. **Bridge them** via Pythonx, HTTP, or message queues

This is closer to a "use the right tool for each layer" story than a "migrate everything" story.

---

## Relevance to difftest

difftest is well-positioned for this space, but the use case is narrower than a full Python-to-Elixir migration wave:

### Strong fit:
- **Hybrid verification:** When teams keep Python for ML but rewrite orchestration in Elixir, difftest can verify the orchestration layer produces identical outputs (API responses, agent decisions, workflow results).
- **Incremental migration:** The Mr. Popov case did a clean-slate rewrite, but most teams would migrate incrementally. difftest can verify each migrated component.
- **CLI/service parity:** `difftest "python3 old_agent.py" "elixir new_agent.exs" --inputs ...` is exactly the use case.
- **The README already mentions this:** "Moving from Python to Elixir?" is listed as a use case. Good instinct.

### Limitations:
- Agent orchestration systems are often stateful, long-running, and event-driven -- harder to test with simple input/output comparison than pure functions.
- Many migrations are to hybrid architectures, not full rewrites, making the "two programs, same input, compare output" model less directly applicable.
- The volume of actual migrations is currently low. This is a forward-looking use case.

### Recommendation:
If Python-to-Elixir migrations accelerate (and OpenAI's Symphony may catalyze that), difftest could be a compelling tool in the migration toolkit. A worked example showing `difftest "python3 agent.py" "elixir agent.exs"` with a simple agent scenario would make this concrete for potential users.

---

## Key Sources

- [Your Agent Framework Is Just a Bad Clone of Elixir](https://georgeguimaraes.com/your-agent-orchestrator-is-just-a-bad-clone-of-elixir/) -- George Guimaraes
- [OpenAI Symphony GitHub](https://github.com/openai/symphony)
- [Python React to Elixir Phoenix Migration Breakdown](https://mrpopov.com/posts/python-react-to-elixir-phoenix-migration-breakdown/) -- Mr. Popov
- [Why Elixir is Perfect for Building AI Agents at Scale](https://elixirator.com/blog/elixir-for-ai-agents/) -- Elixirator
- [Embedding Python in Elixir, it's Fine](https://dashbit.co/blog/running-python-in-elixir-its-fine) -- Dashbit
- [How Elixir Helped Us Integrate AI Agents](https://folderit.net/how-elixir-helped-us-integrate-ai-agents-at-lightning-speed/) -- Folder IT
- [Elixir Production Adoption by Top Companies](https://www.curiosum.com/blog/adoption-of-elixir-by-top-companies) -- Curiosum
- [OpenAI Symphony Elixir Forum Discussion](https://elixirforum.com/t/openai-released-a-library-that-uses-elixir-to-orchestrate-ai-agents/74520)
- [Awesome ML/Gen AI Elixir](https://github.com/georgeguimaraes/awesome-ml-gen-ai-elixir)
- [I Built a Platform Supporting Millions of AI Agents in Elixir](https://guycoding.medium.com/i-built-a-platform-supporting-millions-of-ai-agents-in-elixir-a72f133955ee)
