import { Dashboard } from "@/components/Dashboard";
import { Card, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  return (
    <Dashboard>
      <div className="mx-auto max-w-5xl w-full flex flex-col gap-8">
        <div className="text-center mt-16">
          <p className="text-4xl">Quote Engine</p>
          <p>Initialised.</p>
        </div>
        <div className="flex flex-col gap-4">
          {/* --------------------------------------- */}
          {/* THIS POLISH PART FOR INITIAL DEPLOYMENT */}
          {/* -- TO BE REPLACED  BY DATABASE FETCH -- */}
          {/* --------------------------------------- */}
          <h3 className="text-xl font-bold">{"Statystyki"}</h3>
          <div className="flex flex-col md:flex-row gap-4">
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                {"..."}
              </CardTitle>
              {"quotes in the database"}
            </Card>
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                {"∞"}
              </CardTitle>
              {"curses, insults, and bad words"}
            </Card>
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                {"..."}
              </CardTitle>
              {"quotes added today"}
            </Card>
          </div>
        </div>
      </div>
    </Dashboard>
  );
}
