<template>
  <div>
    <h1 class="text-h4 mb-4">Dodaj kontrahenta</h1>
    
    <v-card>
      <v-card-title>
        Nowy kontrahent
      </v-card-title>
      
      <v-card-text>
        <v-form ref="form" v-model="formValid" @submit.prevent="handleSubmit">
          <v-row>
            <v-col cols="12" md="6">
              <v-text-field
                v-model="formData.name"
                label="Nazwa kontrahenta"
                :rules="fieldsTouched.name ? nameRules : []"
                variant="outlined"
                required
                clearable
                @blur="fieldsTouched.name = true"
                @input="fieldsTouched.name && validateField()"
              ></v-text-field>
            </v-col>
            
            <v-col cols="12" md="6">
              <v-text-field
                v-model="formData.nip"
                label="NIP"
                :rules="fieldsTouched.nip ? nipRules : []"
                variant="outlined"
                required
                clearable
                placeholder="1234567890"
                maxlength="10"
                @blur="fieldsTouched.nip = true"
                @input="fieldsTouched.nip && validateField()"
              ></v-text-field>
            </v-col>
          </v-row>
        </v-form>
      </v-card-text>
      
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="error"
          variant="flat"
          @click="handleCancel"
        >
          Anuluj
        </v-btn>
        <v-btn
          color="success"
          variant="flat"
          :disabled="!formValid"
          :loading="loading"
          @click="handleSubmit"
        >
          Zapisz
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';

interface ContractorFormData {
  name: string;
  nip: string;
}

const form = ref();
const formValid = ref(false);
const loading = ref(false);

const formData = reactive<ContractorFormData>({
  name: '',
  nip: ''
});

// Track which fields have been blurred
const fieldsTouched = reactive({
  name: false,
  nip: false
});

const nameRules = [
  (v: string) => !!v || 'Nazwa jest wymagana',
  (v: string) => (v && v.length >= 2) || 'Nazwa musi mieć co najmniej 2 znaki',
  (v: string) => (v && v.length <= 100) || 'Nazwa nie może być dłuższa niż 100 znaków'
];

const nipRules = [
  (v: string) => !!v || 'NIP jest wymagany',
  (v: string) => /^\d{10}$/.test(v) || 'NIP musi składać się z 10 cyfr',
  (v: string) => validateNIP(v) || 'Nieprawidłowy numer NIP'
];

// Basic NIP validation (Polish tax identification number)
const validateNIP = (nip: string): boolean => {
  if (!/^\d{10}$/.test(nip)) return false;
  
  const weights = [6, 5, 7, 2, 3, 4, 5, 6, 7];
  let sum = 0;
  
  for (let i = 0; i < 9; i++) {
    sum += parseInt(nip[i]) * weights[i];
  }
  
  const checksum = sum % 11;
  const lastDigit = parseInt(nip[9]);
  
  return checksum === lastDigit;
};

const handleSubmit = async () => {
  const { valid } = await form.value.validate();
  
  if (!valid) return;
  
  loading.value = true;
  
  try {
    // TODO: Replace with actual Tauri API call
    console.log('Creating contractor:', formData);
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Show success message or redirect
    alert('Kontrahent został dodany pomyślnie!');
    
    // Reset form
    resetForm();
    
  } catch (error) {
    console.error('Error creating contractor:', error);
    alert('Wystąpił błąd podczas dodawania kontrahenta');
  } finally {
    loading.value = false;
  }
};

const handleCancel = () => {
  resetForm();
};

const validateField = () => {
  // Trigger validation for the specific field
  form.value?.validate();
};

const resetForm = () => {
  formData.name = '';
  formData.nip = '';
  fieldsTouched.name = false;
  fieldsTouched.nip = false;
  form.value?.resetValidation();
};
</script>

<style scoped>
/* Contractor creation styles */
</style>
