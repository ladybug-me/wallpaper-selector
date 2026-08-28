settings-library-watch-section-desc = Upptäck filer som läggs till utanför skwd-wall och håll biblioteket aktuellt.
settings-library-watch-fallback-label = Reservavläsning
settings-library-watch-fallback-desc = Kontrollera endast biblioteksmappar där inbyggd filbevakning inte fungerar. Aktivera detta för nätverks- eller FUSE-monteringar som missar ändringar och starta sedan om skwd-walld.
settings-library-watch-interval-label = Avläsningsintervall
settings-library-watch-interval-desc = Vänta så här många sekunder mellan begränsade kontroller. Lägre värden hittar ändringar snabbare men läser filsystemet oftare. Starta om skwd-walld efter en ändring.
settings-library-watch-unknown-label = Bevakarstatus saknas
settings-library-watch-unknown-desc = Den här demonen rapporterar inte biblioteksbevakningens status. Uppdatera eller starta om skwd-walld.
settings-library-watch-poll-failed-label = Reservavläsningen kan inte läsa en biblioteksmapp
settings-library-watch-poll-failed-desc = Kontrollera att alla konfigurerade biblioteksmappar är monterade och läsbara. En ny avläsning görs om { $interval } sekunder.
settings-library-watch-polling-label = Reservavläsning aktiv
settings-library-watch-polling-desc = Inbyggd bevakning misslyckades för { $count ->
    [one] en biblioteksmapp
   *[other] { $count } biblioteksmappar
    }. Upp till { $budget } poster kontrolleras med { $interval } sekunders mellanrum. Senaste slutförda synkronisering: { $convergence }.
settings-library-watch-recovering-label = Inbyggd bevakning har återhämtats
settings-library-watch-recovering-desc = Den inbyggda bevakaren är aktiv igen. En fullständig överlämningsskanning pågår innan biblioteket förklaras aktuellt.
settings-library-watch-unavailable-label = Biblioteksbevakning saknas
settings-library-watch-unavailable-desc = Inbyggd filbevakning misslyckades och reservavläsning är avstängd. Aktivera Reservavläsning och starta sedan om skwd-walld.
settings-library-watch-recovered-label = Inbyggd bevakning återställd
settings-library-watch-recovered-desc = Den inbyggda bevakaren och dess överlämningsskanning är aktuella. Senaste slutförda synkronisering: { $convergence }.
settings-library-watch-native-label = Inbyggd filbevakning
settings-library-watch-native-desc = Filsystemshändelser är aktiva för alla biblioteksmappar. Reservavläsningen vilar.
settings-library-watch-convergence-never = Inte slutförd ännu
settings-library-watch-convergence-seconds = för { $value } sekunder sedan
settings-library-watch-convergence-minutes = för { $value } minuter sedan
settings-library-watch-convergence-hours = för { $value } timmar sedan
