export function Sidebar() {
  return (
    <div className="w-64 h-full border-r border-slate-200/20 dark:border-slate-700/30 bg-white/50 dark:bg-slate-900/50 backdrop-blur-md flex flex-col p-4 shadow-lg">
      <div className="flex items-center space-x-3 mb-8">
        <div className="w-8 h-8 rounded-full bg-indigo-500 shadow-lg shadow-indigo-500/50 flex items-center justify-center">
          <span className="text-white font-bold text-sm">N</span>
        </div>
        <h1 className="text-xl font-semibold tracking-tight text-slate-800 dark:text-slate-100">Nautilus</h1>
      </div>
      
      <nav className="flex-1 space-y-2">
        <NavItem label="Dashboard" active />
        <NavItem label="Pipelines" />
        <NavItem label="Cluster Settings" />
      </nav>
    </div>
  );
}

function NavItem({ label, active }: { label: string, active?: boolean }) {
  return (
    <a href="#" className={`block px-4 py-2 rounded-lg transition-all duration-200 ${active ? 'bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 font-medium' : 'text-slate-600 dark:text-slate-400 hover:bg-slate-100/50 dark:hover:bg-slate-800/50 hover:text-slate-900 dark:hover:text-slate-200'}`}>
      {label}
    </a>
  );
}
