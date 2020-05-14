module Module.ThingA where

import Module (commonThing)

thingA :: String
thingA = commonThing ++ "thingA"