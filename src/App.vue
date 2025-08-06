<script setup lang="ts">
import { ref } from "vue";
import Sidebar from "./components/Sidebar.vue";
import NotificationsView from "./components/NotificationsView.vue";
import ContractorsView from "./components/ContractorsView.vue";

const selectedItem = ref("notifications");

const menuItems = [
  {
    title: "Notyfikacje",
    value: "notifications",
    icon: "mdi-bell"
  },
  {
    title: "Kontrahenci",
    value: "contractors",
    icon: "mdi-account-group"
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
            <ContractorsView v-if="selectedItem === 'contractors'" />
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<style scoped>
/* Admin panel styles */
</style>
