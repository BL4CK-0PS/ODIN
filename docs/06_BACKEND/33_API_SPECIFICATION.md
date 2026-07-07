# REST API

Base URL

/api/v1

---

POST

/incidents/upload

Upload logs

Response

Incident ID

---

GET

/incidents/{id}

Retrieve investigation

---

POST

/incidents/search

Similarity search

Response

Top K incidents

---

GET

/incidents/{id}/graph

Knowledge graph

---

GET

/incidents/{id}/timeline

Timeline

---

GET

/incidents/{id}/memory

Memory object

---

POST

/incidents/{id}/explain

Generate explanation

---

GET

/playbooks/{id}

Retrieve playbook

---

GET

/health

Health check

---

Response Format

```
{
    success,

    data,

    metadata,

    error
}
```
