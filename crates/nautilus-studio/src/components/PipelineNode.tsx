import { memo } from 'react';
import { Handle, Position } from '@xyflow/react';

export type NodeStatus = 'pending' | 'running' | 'success' | 'failed';

export interface PipelineNodeData {
  label: string;
  status: NodeStatus;
  duration?: string;
}

const statusColors = {
  pending: 'bg-slate-200 dark:bg-slate-700 text-slate-500 dark:text-slate-400 border-slate-300 dark:border-slate-600',
  running: 'bg-blue-100 dark:bg-blue-900/50 text-blue-700 dark:text-blue-300 border-blue-400 dark:border-blue-500 animate-pulse',
  success: 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-400 border-emerald-400 dark:border-emerald-500',
  failed: 'bg-rose-100 dark:bg-rose-900/30 text-rose-700 dark:text-rose-400 border-rose-400 dark:border-rose-500',
};

const statusIcons = {
  pending: '⏸',
  running: '▶',
  success: '✓',
  failed: '✗',
};

export const PipelineNode = memo(({ data }: { data: PipelineNodeData }) => {
  const colors = statusColors[data.status] || statusColors.pending;
  const icon = statusIcons[data.status] || statusIcons.pending;

  return (
    <div className={`px-4 py-2 shadow-md rounded-xl border-2 backdrop-blur-md transition-all duration-300 min-w-[150px] ${colors}`}>
      <Handle type="target" position={Position.Top} className="w-2 h-2 !bg-slate-400" />
      <div className="flex flex-col">
        <div className="flex items-center justify-between gap-4">
          <div className="font-bold text-sm tracking-wide">{data.label}</div>
          <div className="text-xs">{icon}</div>
        </div>
        {data.duration && (
          <div className="text-xs mt-1 opacity-80 font-mono">
            {data.duration}
          </div>
        )}
      </div>
      <Handle type="source" position={Position.Bottom} className="w-2 h-2 !bg-slate-400" />
    </div>
  );
});
