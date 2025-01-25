// import { DottedEffect } from "@/components/DottedEffect";
import { Quote } from "@/components/Quote";
import { Sidenav } from "@/components/Sidenav";

export default function Home() {
  return (
    <Sidenav>
      <h1 className="text-center text-5xl font-fancy">
        {"Keep note of what people say"}
      </h1>
      {/* <DottedEffect /> */}
      {/*  */}
      <div className="mx-auto max-w-2xl w-full">
        <Quote
          data={{
            lines: [
              {
                id: "1",
                text: "Lewica w kryzysie, ale spoko – Polska też.",
                author: {
                  id: "dziober",
                  name: "dzioba",
                },
              },
              {
                id: "2",
                text: "To mówi wiele o niczym.",
                author: {
                  id: "json",
                  name: "jamesen",
                },
              },
            ],
            clearance: 0,
            likes: 13,
            timestamp: "2024/12/16 @ 19:48",
            context: "W kontekście politycznym...",
          }}
        />
      </div>
    </Sidenav>
  );
}
