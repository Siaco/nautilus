import { useTheme } from '../hooks/useTheme';

export function Header() {
  const { theme, toggleTheme } = useTheme();

  return (
    <header className="h-16 border-b border-slate-200/20 dark:border-slate-700/30 bg-white/50 dark:bg-slate-900/50 backdrop-blur-md flex items-center justify-between px-6 shadow-sm z-10">
      <div className="flex items-center text-sm font-medium text-slate-500 dark:text-slate-400">
        Workspace / Default
      </div>
      
      <div className="flex items-center space-x-4">
        <button 
          onClick={toggleTheme}
          className="p-2 rounded-full hover:bg-slate-200/50 dark:hover:bg-slate-800/50 transition-colors"
          title="Toggle Theme"
        >
          {theme === 'light' ? '🌙' : '☀️'}
        </button>
      </div>
    </header>
  );
}
