import { qfetch } from "@/lib/qfetch";

const CardStat = async ({
  variant,
}: {
  variant:
    | "quoteCount"
    | "weeklyQuoteCount"
    | "monthlyQuoteCount"
    | "userCount"
    | "authorCount"
    | "quotedAuthorCount";
}) => {
  const res = await qfetch(
    variant === "quoteCount"
      ? "/quotes/count"
      : variant === "weeklyQuoteCount"
      ? "/quotes/count/thisweek"
      : variant === "monthlyQuoteCount"
      ? "/quotes/count/thismonth"
      : variant === "userCount"
      ? "/users/count"
      : variant === "authorCount"
      ? "/authors/count"
      : variant === "quotedAuthorCount"
      ? "/authors/quoted-count"
      : "UNREACHABLE",
    {
      cache: "no-store",
    }
  );
  const text = await res.text();
  return text;
};

export { CardStat };
