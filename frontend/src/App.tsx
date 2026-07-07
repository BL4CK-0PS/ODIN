import { useRouter } from "@/lib/router";
import { Layout } from "@/components/Layout";
import { LoadingOverlay } from "@/components/LoadingOverlay";
import { Dashboard } from "@/pages/Dashboard";
import { Investigations } from "@/pages/Investigations";
import { InvestigationDetail } from "@/pages/InvestigationDetail";
import { ThreatMemory } from "@/pages/ThreatMemory";
import { KnowledgeExplorer } from "@/pages/KnowledgeExplorer";
import { SearchPage } from "@/pages/Search";
import { SettingsPage } from "@/pages/Settings";

export function App() {
  const route = useRouter();

  const page = () => {
    switch (route.page) {
      case "dashboard": return <Dashboard />;
      case "investigations": return <Investigations />;
      case "investigation-detail": return <InvestigationDetail id={route.params.id} />;
      case "threat-memory": return <ThreatMemory />;
      case "knowledge-explorer": return <KnowledgeExplorer />;
      case "search": return <SearchPage />;
      case "settings": return <SettingsPage />;
      default: return <Dashboard />;
    }
  };

  return (
    <Layout page={route.page}>
      <LoadingOverlay loading={false} />
      {page()}
    </Layout>
  );
}
