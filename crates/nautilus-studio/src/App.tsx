import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useNodesState, useEdgesState } from "@xyflow/react";
import { AppShell } from "./components/AppShell";
import { PipelineCanvas, initialNodes, initialEdges, getLayoutedElements } from "./components/PipelineCanvas";
import type { ClusterStatus } from "./bindings/ClusterStatus";
import type { LogEvent } from "./bindings/LogEvent";

function App() {
  const [clusterStatus, setClusterStatus] = useState<ClusterStatus | null>(null);
  const [logs, setLogs] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);

  const { nodes: layoutedNodes, edges: layoutedEdges } = getLayoutedElements(initialNodes, initialEdges);
  const [nodes, setNodes, onNodesChange] = useNodesState(layoutedNodes);
  const [edges, , onEdgesChange] = useEdgesState(layoutedEdges);

  useEffect(() => {
    invoke<ClusterStatus>("get_cluster_status")
      .then(setClusterStatus)
      .catch(console.error);

    const unlisten = listen<LogEvent>("pipeline-log", (event) => {
      setLogs((prev) => [...prev, event.payload.line]);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const handleRunPipeline = async () => {
    setIsRunning(true);
    setLogs([]);
    
    // Reset nodes to pending
    setNodes((nds) => nds.map(n => ({ ...n, data: { ...n.data, status: 'pending', duration: undefined } })));

    // Simulate node execution visually bridging with the dummy backend logs
    setTimeout(() => {
      setNodes((nds) => nds.map(n => n.id === 'build' ? { ...n, data: { ...n.data, status: 'running' } } : n));
    }, 500);

    setTimeout(() => {
      setNodes((nds) => nds.map(n => {
        if (n.id === 'build') return { ...n, data: { ...n.data, status: 'success', duration: '1.2s' } };
        if (n.id === 'test' || n.id === 'lint') return { ...n, data: { ...n.data, status: 'running' } };
        return n;
      }));
    }, 2000);

    setTimeout(() => {
      setNodes((nds) => nds.map(n => {
        if (n.id === 'lint') return { ...n, data: { ...n.data, status: 'success', duration: '0.8s' } };
        return n;
      }));
    }, 3500);

    setTimeout(() => {
      setNodes((nds) => nds.map(n => {
        if (n.id === 'test') return { ...n, data: { ...n.data, status: 'success', duration: '2.1s' } };
        if (n.id === 'deploy') return { ...n, data: { ...n.data, status: 'running' } };
        return n;
      }));
    }, 4500);

    setTimeout(() => {
      setNodes((nds) => nds.map(n => {
        if (n.id === 'deploy') return { ...n, data: { ...n.data, status: 'success', duration: '1.5s' } };
        return n;
      }));
      setIsRunning(false);
    }, 5500);

    try {
      await invoke("run_pipeline", { manifestPath: null });
    } catch (e) {
      console.error(e);
      setIsRunning(false);
    }
  };

  return (
    <AppShell>
      <div className="max-w-6xl mx-auto space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700">
        <div className="flex flex-col md:flex-row gap-6">
          <div className="flex-1 p-8 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-xl">
            <div className="flex justify-between items-start">
              <div>
                <h2 className="text-2xl font-semibold mb-2 text-slate-800 dark:text-white tracking-tight">Nautilus Studio</h2>
                <p className="text-slate-600 dark:text-slate-300">
                  Visual Execution Engine
                </p>
              </div>
              <button
                onClick={handleRunPipeline}
                disabled={isRunning}
                className={`px-6 py-3 rounded-xl font-medium transition-all duration-300 shadow-lg ${
                  isRunning 
                    ? 'bg-slate-200 dark:bg-slate-700 text-slate-500 cursor-not-allowed' 
                    : 'bg-indigo-600 hover:bg-indigo-700 text-white hover:shadow-indigo-500/25 hover:-translate-y-0.5'
                }`}
              >
                {isRunning ? 'Pipeline Running...' : 'Run Pipeline'}
              </button>
            </div>
            
            {clusterStatus && (
              <div className="mt-6 p-4 rounded-xl bg-white/50 dark:bg-slate-900/50 border border-indigo-200 dark:border-indigo-900/50">
                <div className="flex items-center justify-between">
                  <h4 className="text-sm font-semibold text-indigo-700 dark:text-indigo-400 uppercase tracking-wider">Cluster Details</h4>
                  <span className="flex items-center gap-1.5 text-xs font-medium px-2 py-1 rounded-full bg-slate-100 dark:bg-slate-800">
                    <div className={`w-1.5 h-1.5 rounded-full ${clusterStatus.is_connected ? 'bg-emerald-500' : 'bg-rose-500'}`}></div>
                    {clusterStatus.is_connected ? 'Connected' : 'Offline'}
                  </span>
                </div>
                <div className="mt-3 flex gap-6 text-sm text-slate-700 dark:text-slate-300">
                  <div className="flex flex-col">
                    <span className="text-xs text-slate-500 mb-0.5">Version</span>
                    <span className="font-mono">{clusterStatus.version}</span>
                  </div>
                  <div className="flex flex-col">
                    <span className="text-xs text-slate-500 mb-0.5">Nodes</span>
                    <span className="font-semibold">{clusterStatus.node_count}</span>
                  </div>
                  <div className="flex flex-col">
                    <span className="text-xs text-slate-500 mb-0.5">Pods</span>
                    <span className="font-semibold">{clusterStatus.pod_count}</span>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div className="lg:col-span-2">
            <PipelineCanvas 
              nodes={nodes} 
              edges={edges} 
              onNodesChange={onNodesChange} 
              onEdgesChange={onEdgesChange} 
            />
          </div>

          <div className="p-6 rounded-2xl bg-slate-900 text-slate-300 font-mono text-xs h-[500px] flex flex-col shadow-inner border border-slate-700/50">
            <h4 className="text-slate-400 uppercase tracking-wider mb-4 pb-2 border-b border-slate-800 font-sans font-semibold">Execution Logs</h4>
            <div className="flex-1 overflow-y-auto space-y-1 pr-2 custom-scrollbar">
              {logs.length === 0 ? (
                <div className="text-slate-600 italic">No execution logs yet. Click 'Run Pipeline' to start.</div>
              ) : (
                logs.map((log, i) => (
                  <div key={i} className="py-1">
                    <span className="text-emerald-500 mr-2">→</span>{log}
                  </div>
                ))
              )}
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}

export default App;
