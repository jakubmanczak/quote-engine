import { Sidenav } from "@/components/Sidenav";
import { QuoteMaker } from "./QuoteMaker";

export default function QuoteCreatePage() {
  return (
    <Sidenav>
      <h1 className="text-center text-5xl font-fancy">{"Quote Maker"}</h1>
      <div className="px-4 max-w-2xl w-full mx-auto">
        <QuoteMaker />
      </div>
    </Sidenav>
  );
}
