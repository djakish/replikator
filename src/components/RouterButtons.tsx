import { Tooltip, Button, Spacer, Grid, useTheme } from "@geist-ui/core";
import { Save, Rewind, Database, Sun, Moon, Info } from "@geist-ui/icons";
import Link from "next/link";
import { Url } from "next/dist/shared/lib/router/router";
import { useEffect, useState } from "react";
import { GridDirection } from "@geist-ui/core/esm/grid/grid-types";
import { usePrefers } from "@/lib/use-prefers";

export default function RouterButtons(href: Url) {
  const theme = useTheme();
  const prefers = usePrefers();

  const [gridDirection, setDirection] = useState<GridDirection>("column");

  const handleResize = () => {
    if (window.innerWidth <= 650) {
      setDirection("row");
    } else {
      setDirection("column");
    }
  };

  useEffect(() => {
    window.addEventListener("resize", handleResize, false);
    if (window.innerWidth <= 650) {
      setDirection("row");
    } else {
      setDirection("column");
    }
  }, []);

  return (
    <Grid xs={24} sm={2} direction={gridDirection}>
      <Tooltip type="lite" placement="right" text={"Backup"}>
        <Link href="/">
          <Button iconRight={<Save />} auto px={0.6} />
        </Link>
      </Tooltip>
      <Spacer h={0.5} />
      <Tooltip type="lite" placement="right" text={"Restore"}>
        <Link href="/restore">
          <Button iconRight={<Rewind />} auto px={0.6} />
        </Link>
      </Tooltip>
      <Spacer h={0.5} />
      <Tooltip type="lite" placement="right" text={"Manage backups"}>
        <Link href="/database">
          <Button iconRight={<Database />} auto px={0.6} />
        </Link>
      </Tooltip>
      <Spacer h={0.5} />
      <Tooltip type="lite" placement="right" text={"Change theme"}>
        <Button
          iconRight={theme.type === "dark" ? <Sun /> : <Moon />}
          auto
          px={0.6}
          onClick={() =>
            prefers.switchTheme(theme.type === "dark" ? "light" : "dark")
          }
        />
      </Tooltip>
      <Spacer h={0.5} />
      <Tooltip type="lite" placement="right" text={"Info"}>
        <Link href="/info">
          <Button iconRight={<Info />} auto px={0.6} />
        </Link>
      </Tooltip>
    </Grid>
  );
}
