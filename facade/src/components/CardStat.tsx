"use client";

import { qfetch } from "@/lib/qfetch";
import { useEffect, useState } from "react";

const CardStat = (props: {
  variant: "quoteCount" | "userCount" | "authorCount";
}) => {
  const [stat, setStat] = useState<string>("...");
  const getQuoteCount = async () => {
    const res = await qfetch("/quotes/count");
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
  useEffect(() => {
    switch (props.variant) {
      case "userCount":
        getUserCount();
        break;
      case "quoteCount":
        getQuoteCount();
        break;
      case "authorCount":
        getAuthorCount();
        break;
    }
  }, []);

  return <>{stat}</>;
};

export { CardStat };
