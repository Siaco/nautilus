import { AppShell } from "./components/AppShell";

function App() {
  return (
    <AppShell>
      <div className="max-w-4xl mx-auto space-y-6 animate-in fade-in slide-in-from-bottom-4 duration-700">
        <div className="p-8 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-xl">
          <h2 className="text-2xl font-semibold mb-2 text-slate-800 dark:text-white tracking-tight">Welcome to Nautilus Studio</h2>
          <p className="text-slate-600 dark:text-slate-300">
            The intelligent pipeline execution engine and Kubernetes client.
          </p>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="p-6 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-lg hover:shadow-xl hover:-translate-y-1 transition-all duration-300 cursor-pointer group">
            <h3 className="text-lg font-medium mb-1 text-slate-800 dark:text-white group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">Run Pipeline</h3>
            <p className="text-sm text-slate-500 dark:text-slate-400">Trigger a new execution workflow.</p>
          </div>
          
          <div className="p-6 rounded-2xl bg-white/40 dark:bg-slate-800/40 backdrop-blur-lg border border-slate-200/50 dark:border-slate-700/50 shadow-lg hover:shadow-xl hover:-translate-y-1 transition-all duration-300 cursor-pointer group">
            <h3 className="text-lg font-medium mb-1 text-slate-800 dark:text-white group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">Cluster Status</h3>
            <p className="text-sm text-slate-500 dark:text-slate-400">View real-time Kubernetes metrics.</p>
          </div>
        </div>
      </div>
    </AppShell>
  );
}

export default App;
