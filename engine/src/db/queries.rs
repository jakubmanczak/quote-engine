pub static ALL_QUOTES_QUERY: &str = "
    SELECT
      quotes.id AS quote_id, quotes.timestamp, quotes.context,
      lines.id AS line_id, lines.content AS line_content,
      lines.position AS linepos,
      authors.id AS author_id, authors.name AS author_name,
      authors.obfname AS author_obfname
    FROM quotes
    JOIN lines ON quotes.id = lines.quote
    LEFT JOIN authors ON lines.author = authors.id
    ORDER BY quotes.id, lines.position";

// pub static QUOTES_QUERY_LIMIT: &str = "
//     SELECT
//       quotes.id AS quote_id, quotes.timestamp, quotes.context,
//       lines.content AS line_content, lines.position AS line_position,
//       authors.id AS author_id, authors.name AS author_name, authors.obfname AS author_obfname
//     FROM quotes JOIN lines ON quotes.id = lines.quote LEFT JOIN authors ON lines.author = authors.id
//     WHERE quotes.id IN (
//       SELECT id FROM quotes ORDER BY timestamp DESC LIMIT 10
//     )
//     ORDER BY quotes.id, lines.position";
