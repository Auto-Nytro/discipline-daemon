import * as Uuid from "../../../../ElementaryTypes/Uuid.ts"
import * as Client from "../../../../Client.ts"
import * as Outcome from "./Outcome.ts"
import * as Password from "../../../../CommonTypes/Password.ts"
import * as Displayer from "../../../../ElementaryTypes/Display.ts"
import * as JsonSerializer from "../../../../ElementaryTypes/JsonSerde/JsonSerializer.ts"
import * as JsonDeserializer from "../../../../ElementaryTypes/JsonSerde/JsonDeserializer.ts"
import * as OperatingSystemUsername from "../../../../CommonTypes/OperatingSystemUsername.ts"
import { Tried } from "../../../../ElementaryTypes/Tried.ts"
import { Unique } from "../../../../ElementaryTypes/Unique.ts";

export type Operation = Unique<"Discipline.NetworkingAccess.Operations.Enabler.ByPassword.MakeEffective", {
  readonly ruleId: Uuid.Uuid
  readonly username: OperatingSystemUsername.OperatingSystemUsername
  readonly password: Password.Password
}>

export function create(
  ruleId: Uuid.Uuid,
  username: OperatingSystemUsername.OperatingSystemUsername,
  password: Password.Password,
): Operation {
  return Unique({
    ruleId,
    username,
    password,
  })
}

export const displayer = Displayer.implement<Operation>(me => 
  Displayer.asNamedObject("Enabler.ByPassword.MakeEffective",
    "ruleId", Uuid.displayer, me.ruleId,
    "username", OperatingSystemUsername.displayer, me.username,
    "password", Password.displayer, me.password,
  )
)

export const jsonSerializer = JsonSerializer.implement<Operation>(me => 
  JsonSerializer.asObject(
    "rule_id", Uuid.jsonSerializer, me.ruleId,
    "username", OperatingSystemUsername.jsonSerializer, me.username,
    "password", Password.jsonSerializer, me.password,
  )
)

export const jsonDeserializer = JsonDeserializer.implement<Operation>(context => 
  Tried.andThen(JsonDeserializer.asObjectContext(context), context => Tried.map3(
    JsonDeserializer.propertyAs(context, "rule_id", Uuid.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "username", OperatingSystemUsername.jsonDeserializer),
    JsonDeserializer.propertyAs(context, "password", Password.jsonDeserializer),
    create
  ))
)

export const executer = Client.Executer.implement(
  [ "NetworkingAccess", "Rules", "Enabler", "ByPassword", "MakeEffective" ],
  create,
  displayer,
  jsonSerializer,
  jsonDeserializer,
  Outcome.displayer,
  Outcome.jsonSerializer,
  Outcome.jsonDeserializer,
)