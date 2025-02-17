# Community-Driven Disaster Preparedness System

## 1. Project Objective
The objective of this project is to develop a **Community-Driven Disaster Preparedness System** that enables communities to collaboratively prepare for natural disasters such as earthquakes, floods, and wildfires. The system allows users to contribute preparedness guides, track emergency resource availability, and coordinate response plans. This project will be built using **Rust** for the backend and **Angular** for the frontend.

## 2. Project Scope
The system will cater to multiple stakeholders:
- **Community Members:** Can contribute preparedness checklists, share local emergency contacts, and access disaster response plans.
- **Local Authorities:** Can verify, update, and distribute real-time alerts and emergency measures.
- **Volunteers & NGOs:** Can track relief resource availability and coordinate distribution efforts.

Key functionalities include **real-time alerts, preparedness guide sharing, emergency resource tracking, and role-based access control**.

## 3. Software Requirements Specification (SRS)
### 3.1 Functional Requirements
- **User Authentication** (Sign Up, Login, Logout)
- **Disaster Preparedness Guides** (Create, Edit, Delete, View)
- **Real-time Emergency Alerts & Notifications** using WebSockets
- **Resource & Shelter Tracking**
- **Community Collaboration & Response Planning**
- **Role-Based Access Control**
- **Responsive UI with Angular Material**
- **State Management using NgRx**

### 3.2 Non-Functional Requirements
- **Scalability:** The system should handle real-time updates for multiple locations.
- **Security:** Secure user data and role-based access with JWT authentication.
- **Performance:** Optimize API calls and implement lazy loading for fast UI interactions.
- **Reliability:** Ensure fault tolerance in database and API handling.

## 4. System Architecture
- **Backend (Rust):**
  - AXUM (API development)
  - MongoDB (Database)
  - Tokio & Warp (WebSockets for real-time updates)
  - JWT (Authentication & Authorization)

- **Frontend (Angular):**
  - Angular 19 (Latest version)
  - Angular Material (UI Components)
  - NgRx (State Management)
  - RxJS (Real-time updates)
  - Angular Guards & Interceptors (Security)

## 5. Key Features & Workflows
### 5.1 User Registration & Authentication
**As a user**, I should be able to register and access location-based disaster preparedness content.

### 5.2 Community Dashboard
**As a logged-in user**, I should be able to view real-time alerts, recommended actions, and nearby shelters.

### 5.3 Disaster Preparedness Guide Management
**As an authority or expert**, I should be able to create, update, and verify preparedness guides for different disasters.

### 5.4 Real-time Emergency Alerts
**As a local authority**, I should be able to send real-time alerts to users in affected locations.

### 5.5 Resource & Shelter Tracking
**As a volunteer or NGO**, I should be able to update real-time availability of essential resources like food, medical aid, and shelter capacity.

### 5.6 Community Collaboration & Response Planning
**As a community member**, I should be able to coordinate with others by joining local response groups and contributing to action plans.

## 6. Deployment & Hosting
- **Backend**: Hosted using **Docker & Rust's AXUM server**.
- **Frontend**: Deployed on **Firebase or Vercel**.
- **Database**: Mongo hosted on **ATLAS**. 

## 7. Task Submission Guidelines
- Submit a **GitHub repository** with proper documentation.
- Ensure code quality by using **TypeScript best practices & Rust Analyzer**.
- Deployment should be fully functional for demonstration.

---
This project provides interns with **hands-on experience** in Rust-based backend development and Angular frontend engineering while contributing to a socially impactful disaster preparedness initiative.


## Contributors
- **Lakshya** - Shelter Module
- **Raj** - User Module
- **Madhav** - Middleware Module
- **Krunal** - Live API Module
- **Mitanshu** - Disaster Module
- **Smit** - Resources Module
- **Jay P** - web socket (pending)


