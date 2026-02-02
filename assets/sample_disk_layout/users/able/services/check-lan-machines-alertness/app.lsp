;; Fired when a service is started.
(service-on-start 
 (@supervisor-spawn app-lan-alertness))

;; Fired when a service is reset
(service-on-reset ())

;; Fired when a service shuts down.
(service-on-stop ())

;; Meant to check the lan network for machines nearby.
(app-lan-alertness (
 (defv running true)
 (while running (
  (sleep 10)
  (network-ping-lan)))
))