import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { AppShell } from "./components/AppShell";
import type { ClusterStatus } from "./bindings/ClusterStatus";
import type { LogEvent } from "./bindings/LogEvent";

function App() {
  const [clusterStatus, setClusterStatus] = useState<ClusterStatus | null>(null);
  const [logs, setLogs] = useState<string[]>([]);
  const [isRunning, setIsRunning] = useState(false);

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
    try {
      await invoke("run_pipeline", { manifestPath: null });
    } catch (e) {
      console.error(e);
      setIsRunning(false);
    }
  };

  return (
    <AppShell>
      <div className="max-w-4xl mx-auto space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700">
        <div className="p-8 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-xl">
          <h2 className="text-2xl font-semibold mb-2 text-slate-800 dark:text-white tracking-tight">Welcome to Nautilus Studio</h2>
          <p className="text-slate-600 dark:text-slate-300">
            The intelligent pipeline execution engine and Kubernetes client.
          </p>
          
          {clusterStatus && (
            <div className="mt-4 p-4 rounded-lg bg-white/50 dark:bg-slate-900/50 border border-indigo-200 dark:border-indigo-900/50">
              <h4 className="text-sm font-semibold text-indigo-700 dark:text-indigo-400 uppercase tracking-wider mb-2">Cluster Status</h4>
              <div className="flex gap-4 text-sm text-slate-700 dark:text-slate-300">
                <span>Version: <span className="font-mono bg-slate-200 dark:bg-slate-800 px-1 rounded">{clusterStatus.version}</span></span>
                <span>Nodes: <strong>{clusterStatus.node_count}</strong></span>
                <span>Pods: <strong>{clusterStatus.pod_count}</strong></span>
                <span className="flex items-center gap-1">
                  <div className={`w-2 h-2 rounded-full ${clusterStatus.is_connected ? 'bg-green-500' : 'bg-red-500'}`}></div>
                  {clusterStatus.is_connected ? 'Connected' : 'Disconnected'}
                </span>
              </div>
            </div>
          )}
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div 
            onClick={handleRunPipeline}
            className={`p-6 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-lg transition-all duration-300 cursor-pointer group ${isRunning ? 'opacity-50 pointer-events-none' : 'hover:shadow-xl hover:-translate-y-1'}`}
          >
            <h3 className="text-lg font-medium mb-1 text-slate-800 dark:text-white group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">
              {isRunning ? 'Pipeline Running...' : 'Run Pipeline'}
            </h3>
            <p className="text-sm text-slate-500 dark:text-slate-400">Trigger a new execution workflow.</p>
          </div>
        </div>

        {logs.length > 0 && (
          <div className="p-6 rounded-2xl bg-slate-900 text-slate-300 font-mono text-sm h-64 overflow-y-auto shadow-inner border border-slate-700/50">
            {logs.map((log, i) => (
              <div key={i} className="py-1 border-b border-slate-800/50 last:border-0">{log}</div>
            ))}
          </div>
        )}
      </div>
    </AppShell>
  );
}

export default App;
