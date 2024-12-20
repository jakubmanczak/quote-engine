import { CardStat } from "@/components/CardStat";
import { Dashboard } from "@/components/Dashboard";
import { Card, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  return (
    <Dashboard>
      <div className="mx-auto max-w-5xl w-full flex flex-col gap-8">
        <div className="text-center mt-16">
          <p className="text-4xl">Quote Engine</p>
        </div>
        <div className="flex flex-col gap-4">
          <h3 className="text-xl font-bold">{"Quick Stats"}</h3>
          <div className="flex flex-col lg:flex-row gap-4">
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                <CardStat variant="quoteCount" />
              </CardTitle>
              {"quotes total"}
            </Card>
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                <CardStat variant="quotedAuthorCount" />
              </CardTitle>
              {"people quoted"}
            </Card>
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                <CardStat variant="userCount" />
              </CardTitle>
              {"registered users"}
            </Card>
          </div>
          <div className="flex flex-col md:flex-row gap-4">
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                <CardStat variant="weeklyQuoteCount" />
              </CardTitle>
              {"quotes added this week"}
            </Card>
            <Card className="p-4 flex-1 text-center">
              <CardTitle className="text-3xl font-bold font-sans">
                <CardStat variant="monthlyQuoteCount" />
              </CardTitle>
              {"quotes added this month"}
            </Card>
          </div>
        </div>
      </div>
    </Dashboard>
  );
}
