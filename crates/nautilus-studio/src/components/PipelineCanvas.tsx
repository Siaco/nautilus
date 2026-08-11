import { useMemo } from 'react';
import { ReactFlow, Controls, Background, BackgroundVariant, NodeChange, EdgeChange, Node, Edge } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import dagre from 'dagre';
import { PipelineNode } from './PipelineNode';

const initialNodes: Node[] = [
  { id: 'build', type: 'pipelineNode', data: { label: 'build', status: 'pending' }, position: { x: 0, y: 0 } },
  { id: 'test', type: 'pipelineNode', data: { label: 'test', status: 'pending' }, position: { x: 0, y: 0 } },
  { id: 'lint', type: 'pipelineNode', data: { label: 'lint', status: 'pending' }, position: { x: 0, y: 0 } },
  { id: 'deploy', type: 'pipelineNode', data: { label: 'deploy', status: 'pending' }, position: { x: 0, y: 0 } },
];

const initialEdges: Edge[] = [
  { id: 'e-build-test', source: 'build', target: 'test', animated: true },
  { id: 'e-build-lint', source: 'build', target: 'lint', animated: true },
  { id: 'e-test-deploy', source: 'test', target: 'deploy', animated: true },
  { id: 'e-lint-deploy', source: 'lint', target: 'deploy', animated: true },
];

const getLayoutedElements = (nodes: Node[], edges: Edge[], direction = 'TB') => {
  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  
  const nodeWidth = 172;
  const nodeHeight = 56;
  
  dagreGraph.setGraph({ rankdir: direction });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  const layoutedNodes = nodes.map((node) => {
    const nodeWithPosition = dagreGraph.node(node.id);
    return {
      ...node,
      targetPosition: 'top',
      sourcePosition: 'bottom',
      position: {
        x: nodeWithPosition.x - nodeWidth / 2,
        y: nodeWithPosition.y - nodeHeight / 2,
      },
    };
  });

  return { nodes: layoutedNodes as Node[], edges };
};

export function PipelineCanvas({ nodes, edges, onNodesChange, onEdgesChange }: {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: (changes: NodeChange[]) => void;
  onEdgesChange: (changes: EdgeChange[]) => void;
}) {
  const nodeTypes = useMemo(() => ({ pipelineNode: PipelineNode }), []);

  return (
    <div className="w-full h-[500px] border border-slate-200 dark:border-slate-700/50 rounded-2xl overflow-hidden bg-slate-50/50 dark:bg-slate-900/50 backdrop-blur-sm shadow-inner">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        nodeTypes={nodeTypes}
        fitView
        className="dark:bg-slate-900/10"
      >
        <Controls className="bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-700 text-slate-800 dark:text-slate-200 fill-slate-800 dark:fill-slate-200" />
        <Background variant={BackgroundVariant.Dots} gap={12} size={1} color="currentColor" className="text-slate-300 dark:text-slate-700" />
      </ReactFlow>
    </div>
  );
}

export { initialNodes, initialEdges, getLayoutedElements };
