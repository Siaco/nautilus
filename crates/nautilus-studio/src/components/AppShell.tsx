import { Sidebar } from './Sidebar';
import { Header } from './Header';
import type { PipelineInfo } from '../bindings/PipelineInfo';

export function AppShell({ 
  children,
  pipelines,
  activePipeline,
  onSelectPipeline,
  onRefreshPipelines
}: { 
  children: React.ReactNode,
  pipelines: PipelineInfo[],
  activePipeline: string | null,
  onSelectPipeline: (path: string) => void,
  onRefreshPipelines: () => void
}) {
  return (
    <div className="flex h-screen w-full overflow-hidden bg-transparent font-sans">
      <Sidebar 
        pipelines={pipelines} 
        activePipeline={activePipeline} 
        onSelectPipeline={onSelectPipeline} 
        onRefreshPipelines={onRefreshPipelines} 
      />
      <div className="flex-1 flex flex-col relative bg-white/30 dark:bg-slate-900/30">
        <Header />
        <main className="flex-1 overflow-auto p-6 z-0">
          {children}
        </main>
      </div>
    </div>
  );
}
