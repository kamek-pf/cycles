module Module.ThingB where

import Module (commonThing)

thingB :: String
thingB = commonThing ++ "thingB"