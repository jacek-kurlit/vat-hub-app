<script setup lang="ts">
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import NotificationsView from "./components/NotificationsView.vue";
import ContractorsView from "./components/ContractorsView.vue";
import ContractorsCreateView from "./components/ContractorsCreateView.vue";
import ContractorsImportView from "./components/ContractorsImportView.vue";
import { useAlerts } from "./composables/useAlerts";

const { alerts, removeAlert } = useAlerts();

const selectedItem = ref("contractors-list");

const menuItems = [
  {
    title: "Notyfikacje",
    value: "notifications",
    icon: "mdi-bell"
  },
  {
    title: "Kontrahenci",
    value: "contractors",
    icon: "mdi-account-group",
    children: [
      {
        title: "Lista",
        value: "contractors-list",
        icon: "mdi-format-list-bulleted"
      },
      {
        title: "Dodaj",
        value: "contractors-create",
        icon: "mdi-plus"
      },
      {
        title: "Import",
        value: "contractors-import",
        icon: "mdi-import"
      }
    ]
  }
];

const selectMenuItem = (item: string) => {
  selectedItem.value = item;
};
</script>

<template>
  <v-app>
    <Sidebar 
      :selected-item="selectedItem"
      :menu-items="menuItems"
      @select-item="selectMenuItem"
    />

    <v-main>
      <v-container fluid>
        <v-row>
          <v-col cols="12">
            <NotificationsView v-if="selectedItem === 'notifications'" />
            <ContractorsView v-if="selectedItem === 'contractors-list'" />
            <ContractorsCreateView 
              v-if="selectedItem === 'contractors-create'" 
              @navigate-to-list="selectMenuItem('contractors-list')"
            />
            <ContractorsImportView v-if="selectedItem === 'contractors-import'" />
          </v-col>
        </v-row>
      </v-container>
    </v-main>

    <!-- Global Alerts Container -->
    <div class="alerts-container">
      <v-slide-x-reverse-transition group>
        <v-alert
          v-for="alert in alerts"
          :key="alert.id"
          :type="alert.type"
          :title="alert.title"
          :text="alert.text"
          closable
          class="alert-item mb-2"
          @click:close="removeAlert(alert.id)"
        ></v-alert>
      </v-slide-x-reverse-transition>
    </div>
  </v-app>
</template>

<style scoped>
/* Admin panel styles */
.alerts-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  max-width: 400px;
  width: 100%;
  pointer-events: none;
}

.alert-item {
  pointer-events: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

@media (max-width: 768px) {
  .alerts-container {
    left: 20px;
    right: 20px;
    max-width: none;
  }
}
</style>
