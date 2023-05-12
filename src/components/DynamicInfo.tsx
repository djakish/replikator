import { Card, Page, Text, Grid, Divider } from "@geist-ui/core";
import RouterButtons from "@/components/RouterButtons";
import Controls from "./Controls";

export default function DynamicInfo() {
  return (
    <Page render="effect-seo">
      <Controls />
      <Grid.Container gap={2} justify="flex-start">
        <RouterButtons />
        <Grid sm={22}>
          <Card hoverable shadow width="100%">
            <Text h2 my={0}>
              About the app
            </Text>
            <Divider my={2} h={4} />
            <Text h3 p>
              How to use the app?{" "}
            </Text>
            <Text p>
              Create either a single time or updatable backup. Then files will
              be compressed to a destination. To restore select a directory of
              the backup{" "}
            </Text>
            <Divider my={2} h={4} />
            <Text h3 p>
              How to use Google Drive or One Drive?{" "}
            </Text>
            <Text p>
              {" "}
              Install GDrive or OneDrive apps, which will create a virtual drive
              on your PC, then you will be able to make a backup to them.{" "}
            </Text>
          </Card>
        </Grid>
      </Grid.Container>
    </Page>
  );
}
