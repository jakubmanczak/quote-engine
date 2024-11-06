"use client";

import { qfetch } from "@/lib/qfetch";
import { useEffect, useState } from "react";

const CardStat = (props: {
  variant:
    | "quoteCount"
    | "weeklyQuoteCount"
    | "userCount"
    | "authorCount"
    | "quotedAuthorCount";
}) => {
  const [stat, setStat] = useState<string>("...");
  const getQuoteCount = async () => {
    const res = await qfetch("/quotes/count");
    if (res.ok) {
      const text = await res.text();
      setStat(text);
    } else setStat("err");
  };
  const getWeeklyQuoteCount = async () => {
    const res = await qfetch("/quotes/count/thisweek");
    if (res.ok) {
      const text = await res.text();
      setStat(text);
    } else setStat("err");
  };
  const getUserCount = async () => {
    const res = await qfetch("/users/count");
    if (res.ok) {
      const text = await res.text();
      setStat(text);
    } else setStat("err");
  };
  const getAuthorCount = async () => {
    const res = await qfetch("/authors/count");
    if (res.ok) {
      const text = await res.text();
      setStat(text);
    } else setStat("err");
  };
  const getQuotedAuthorCount = async () => {
    const res = await qfetch("/authors/quoted-count");
    if (res.ok) {
      const text = await res.text();
      setStat(text);
    } else setStat("err");
  };
  useEffect(() => {
    switch (props.variant) {
      case "userCount":
        getUserCount();
        break;
      case "quoteCount":
        getQuoteCount();
        break;
      case "weeklyQuoteCount":
        getWeeklyQuoteCount();
        break;
      case "authorCount":
        getAuthorCount();
        break;
      case "quotedAuthorCount":
        getQuotedAuthorCount();
        break;
    }
  }, []);

  return <>{stat}</>;
};

export { CardStat };
