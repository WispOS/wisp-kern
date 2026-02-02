;; Fired when a service is started.
(service-on-start 
 (@supervisor-spawn vfs-cache-sync))

;; Fired when a service is reset
(service-on-reset ())

;; Fired when a service shuts down.
(service-on-stop ())

;; Meant to check the lan network for machines nearby.
(vfs-cache-sync (
 (defv running true)
 (defv sleep-timeout 
  (@config vfs-cache-sync-time))

 (while running (
  (sleep sleep-timeout)
  (@mailbox-send 
   (@workspace "vfs")
   `(disk-sync)
  )))
))