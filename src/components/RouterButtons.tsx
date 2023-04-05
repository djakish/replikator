import { Tooltip, Button, Spacer, Grid } from "@geist-ui/core";
import { Save, Rewind, Database } from "@geist-ui/icons";
import Link from 'next/link'
import { Url } from "next/dist/shared/lib/router/router";

export default function RouterButtons(href: Url) {

    return (
        <Grid sm={2} direction='column'>
          <Tooltip type="lite" placement="right" text={'Backup'}>
            <Link href="/">
              <Button iconRight={<Save />} auto px={0.6} />
            </Link>
          </Tooltip>
          <Spacer h={0.5} />
          <Tooltip type="lite" placement="right" text={'Restore'}>
            <Link href="/restore">
              <Button iconRight={<Rewind />} auto px={0.6} />
            </Link>
          </Tooltip>
          <Spacer h={0.5} />
          <Tooltip type="lite" placement="right" text={'Manage backups'}>
            <Link href="/">
              <Button iconRight={<Database />} auto px={0.6} />
            </Link>
          </Tooltip>
        </Grid>
    )
}
