import type { PipelineInfo } from '../bindings/PipelineInfo';

export function Sidebar({ 
  pipelines, 
  activePipeline, 
  onSelectPipeline,
  onRefreshPipelines 
}: { 
  pipelines: PipelineInfo[], 
  activePipeline: string | null,
  onSelectPipeline: (path: string) => void,
  onRefreshPipelines: () => void
}) {
  return (
    <div className="w-64 h-full border-r border-slate-200/20 dark:border-slate-700/30 bg-white/50 dark:bg-slate-900/50 backdrop-blur-md flex flex-col p-4 shadow-lg">
      <div className="flex items-center space-x-3 mb-8">
        <div className="w-8 h-8 rounded-full bg-indigo-500 shadow-lg shadow-indigo-500/50 flex items-center justify-center">
          <span className="text-white font-bold text-sm">N</span>
        </div>
        <h1 className="text-xl font-semibold tracking-tight text-slate-800 dark:text-slate-100">Nautilus</h1>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        <div className="flex items-center justify-between mb-2 px-2">
          <h2 className="text-xs font-bold text-slate-500 uppercase tracking-wider">Pipelines</h2>
          <button onClick={onRefreshPipelines} className="text-xs text-indigo-500 hover:text-indigo-400 font-medium">Refresh</button>
        </div>
        <nav className="space-y-1">
          {pipelines.length === 0 ? (
             <div className="text-sm text-slate-500 italic px-2 mt-2">No .yml files found in workspace</div>
          ) : (
            pipelines.map((p) => (
              <a 
                key={p.path}
                href="#" 
                onClick={(e) => { e.preventDefault(); onSelectPipeline(p.path); }}
                className={`block px-3 py-2 rounded-lg text-sm transition-all duration-200 truncate ${activePipeline === p.path ? 'bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 font-medium' : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100/50 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-slate-200'}`}
                title={p.name}
              >
                {p.name}
              </a>
            ))
          )}
        </nav>
      </div>
    </div>
  );
}
