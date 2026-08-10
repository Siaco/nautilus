import { Sidebar } from './Sidebar';
import { Header } from './Header';

export function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex h-screen w-full overflow-hidden bg-transparent font-sans">
      <Sidebar />
      <div className="flex-1 flex flex-col relative bg-white/30 dark:bg-slate-900/30">
        <Header />
        <main className="flex-1 overflow-auto p-6 z-0">
          {children}
        </main>
      </div>
    </div>
  );
}
