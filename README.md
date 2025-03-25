//test
GET /message_types
POST /message_types
GET /message_types/:id
PUT /message_types/:id
DELETE /message_types/:id

GET /business_units
POST /business_units
GET /business_units/:id
PUT /business_units/:id
DELETE /business_units/:id

GET /business_units/:id/channels
POST /business_units/:id/channels
GET /business_units/:id/channels/:id
PUT /business_units/:id/channels/:id
DELETE /business_units/:id/channels/:id

GET /business_units/:id/messages
GET /business_units/:id/messages/:id

GET /business_units/:id/message_types/:id/templates
POST /business_units/:id/message_types/:id/templates
GET /business_units/:id/message_types/:id/templates/:id
PUT /business_units/:id/message_types/:id/templates/:id
DELETE /business_units/:id/message_types/:id/templates/:id

GET /business_units/:id/message_types/:id/templates/:id/sms
PUT /business_units/:id/message_types/:id/templates/:id/sms
DELETE /business_units/:id/message_types/:id/templates/:id/sms

GET /business_units/:id/message_types/:id/templates/:id/email
PUT /business_units/:id/message_types/:id/templates/:id/email
DELETE /business_units/:id/message_types/:id/templates/:id/email

GET /business_units/:id/message_types/:id/templates/:id/push
PUT /business_units/:id/message_types/:id/templates/:id/push
DELETE /business_units/:id/message_types/:id/templates/:id/push

POST /messages
GET /messages/:id

GET /business_units/:id/message_types/:id/template_assignments/:id/template/:type
PUT /business_units/:id/message_types/:id/template_assignments/:id/template/:type
DELETE /business_units/:id/message_types/:id/template_assignments/:id/template/:type
