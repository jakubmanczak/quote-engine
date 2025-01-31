"use client";
import { GrainEffect } from "@/components/GrainEffect";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { LucideQuote } from "lucide-react";
import { useEffect, useState } from "react";

type QuoteCreateData = {
  lines: {
    id: string;
    content: string;
    author_id: string;
  }[];
  context: string;
  timestamp: string;
  clearance: number;
};

const emptyLine = {
  id: "00000000-0000-0000-0000-000000000000",
  author_id: "00000000-0000-0000-0000-000000000000",
  content: "",
};

const currentNaiveDateTime = () => {
  const date = new Date();
  const y = date.getFullYear();
  let mo = (date.getMonth() + 1).toString(); // this is atrocious
  if (mo.length === 1) {
    mo = `0${mo}`;
  }
  let d = date.getDate().toString();
  if (d.length === 1) {
    d = `0${d}`;
  }
  let h = date.getHours().toString();
  if (h.length === 1) {
    h = `0${h}`;
  }
  let mi = date.getMinutes().toString();
  if (mi.length === 1) {
    mi = `0${mi}`;
  }
  let s = date.getSeconds().toString();
  if (s.length === 1) {
    s = `0${s}`;
  }
  // actually, all of it is atrocious

  return `${y}-${mo}-${d}T${h}:${mi}:${s}`;
};

export const QuoteMaker = () => {
  const firstLoadDateTime = currentNaiveDateTime();
  const [quoteData, setQuoteData] = useState<QuoteCreateData>({
    lines: [emptyLine],
    timestamp: firstLoadDateTime,
    clearance: 0,
    context: "",
  });

  useEffect(() => {
    let idx = 0;
    for (const line of quoteData.lines) {
      const isLast = idx === quoteData.lines.length - 1;

      if (isLast && line.content.length !== 0) {
        // if the last line is not empty, add a new line
        setQuoteData({
          ...quoteData,
          lines: [...quoteData.lines, emptyLine],
        });
      }

      idx++;
    }
  }, [quoteData]);
  return (
    <div className="p-4 bg-sidebar border rounded-md backdrop-blur relative overflow-clip">
      <GrainEffect />
      <div className="flex flex-col gap-4">
        {quoteData.lines.map((el, idx) => {
          const isLast = idx === quoteData.lines.length - 1;
          const isFirst = idx === 0;
          return (
            <div key={`line-${idx}`}>
              <div className="flex flex-row items-center gap-2">
                <LucideQuote className="text-muted" />
                <Input
                  type="text"
                  value={el.content}
                  onChange={(e) => {
                    const newlines = quoteData.lines;
                    newlines[idx] = {
                      ...newlines[idx],
                      content: e.target.value,
                    };
                    setQuoteData({ ...quoteData, lines: newlines });
                  }}
                  className="w-full mb-2"
                  placeholder={
                    isFirst
                      ? "Write text here to start!"
                      : "Empty lines won't be sent."
                  }
                />
              </div>
              <div className="flex flex-row gap-4 justify-end">
                <Input
                  type="text"
                  placeholder="Author ID"
                  className="min-w-64 w-fit"
                />
                <Button
                  variant={"secondary"}
                  disabled={
                    isLast && el.content.length === 0
                    //   (isFirst && quoteData.lines.length - 1 <= 2)
                  }
                  onClick={() => {
                    const newlines = quoteData.lines;
                    newlines.splice(idx, 1);
                    setQuoteData({ ...quoteData, lines: newlines });
                  }}
                >
                  {"Delete"}
                </Button>
              </div>
            </div>
          );
        })}
      </div>
      <hr className="my-4" />
      <div className="grid grid-cols-2 gap-4">
        <div>
          <p className="mb-2">
            {"Timestamp"}
            <span className="text-middleground text-xs">
              {" (must match format)"}
            </span>
          </p>
          <Input
            type="text"
            value={quoteData.timestamp}
            onChange={(e) => {
              setQuoteData({ ...quoteData, timestamp: e.target.value });
            }}
          />
        </div>
        <div>
          <p className="mb-2">
            {"Context"}
            <span className="text-middleground text-xs">
              {" (must be kept short)"}
            </span>
          </p>
          <Input
            type="text"
            value={quoteData.context}
            onChange={(e) => {
              setQuoteData({ ...quoteData, context: e.target.value });
            }}
            placeholder={`e.g. "About the city council"`}
          />
        </div>
      </div>
      <hr className="my-4" />
      <div className="flex flex-row justify-between items-center">
        <div>
          <p>{"Required clearance"}</p>
          <p className="text-middleground text-xs">{"(0 is public)"}</p>
        </div>
        <Input
          type="number"
          min={0}
          max={255}
          value={quoteData.clearance}
          onChange={(e) => {
            setQuoteData({ ...quoteData, clearance: parseInt(e.target.value) });
          }}
          className="max-w-32"
        />
      </div>
      <div className="flex flex-row gap-4">
        <Input
          type="range"
          min={0}
          max={255}
          value={quoteData.clearance}
          onChange={(e) => {
            setQuoteData({ ...quoteData, clearance: parseInt(e.target.value) });
          }}
        />
      </div>
      <Button className="w-full mt-6" variant={"secondary"}>
        Submit quote
      </Button>
    </div>
  );
};
