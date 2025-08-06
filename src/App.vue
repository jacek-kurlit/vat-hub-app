<script setup lang="ts">
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import NotificationsView from "./components/NotificationsView.vue";
import ContractorsView from "./components/ContractorsView.vue";
import ContractorsCreateView from "./components/ContractorsCreateView.vue";
import ContractorsImportView from "./components/ContractorsImportView.vue";

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
            <ContractorsCreateView v-if="selectedItem === 'contractors-create'" />
            <ContractorsImportView v-if="selectedItem === 'contractors-import'" />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
/* Admin panel styles */
</style>
