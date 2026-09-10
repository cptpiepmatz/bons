import { format } from "sql-formatter";
import { expandGlob } from "@std/fs";

for await (const { path } of expandGlob("backend/**/*.sql")) {
  const sql = await Deno.readTextFile(path);
  const formatted = format(sql, {
    language: "sqlite",
    keywordCase: "upper",
    tabWidth: 2,
    useTabs: false,
  });

  await Deno.writeTextFile(path, formatted);
}
