import { AppSyncResolverEvent } from "aws-lambda";
import {
  QueryEventGetArgs,
  MutationEventCreateArgs,
  Event,
} from "../../graphql/graphql";

export const handler = async (
  event: AppSyncResolverEvent<
    QueryEventGetArgs | MutationEventCreateArgs,
    Event
  >,
) => {
  console.log("Event", event);
  switch (event.info.fieldName) {
    case "eventGet":
      return {
        id: (event.arguments as QueryEventGetArgs).id,
        name: "testname",
        createdAt: new Date(Date.now()).toLocaleString(),
      };
    default:
      return {};
  }
};
