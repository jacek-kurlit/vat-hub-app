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
            <v-col cols="12" md="8">
              <v-text-field
                v-model="formData.nip"
                label="NIP"
                :rules="fieldsTouched.nip ? nipRules : []"
                variant="outlined"
                required
                clearable
                placeholder="1234567890"
                maxlength="10"
                :disabled="dataFetched"
                @blur="fieldsTouched.nip = true"
                @input="fieldsTouched.nip && validateField()"
                @keyup.enter="handleNipEnter"
              ></v-text-field>
            </v-col>
            
            <v-col cols="12" md="4" class="d-flex align-center">
              <v-btn
                color="primary"
                variant="flat"
                :disabled="!formData.nip || !validateNIP(formData.nip) || dataFetched"
                :loading="fetchingData"
                @click="fetchContractorData"
              >
                Pobierz dane
              </v-btn>
            </v-col>
          </v-row>

          <div v-if="dataFetched">
            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formData.name"
                  label="Nazwa kontrahenta"
                  variant="outlined"
                  readonly
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formData.vatStatus"
                  label="Status VAT"
                  variant="outlined"
                  readonly
                ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formData.regon"
                  label="REGON"
                  variant="outlined"
                  readonly
                ></v-text-field>
              </v-col>
              
              <v-col cols="12" md="6">
                <v-text-field
                  v-model="formData.krs"
                  label="KRS"
                  variant="outlined"
                  readonly
                ></v-text-field>
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="12">
                <v-textarea
                  v-model="formData.residenceAddress"
                  label="Adres siedziby"
                  variant="outlined"
                  readonly
                  rows="3"
                ></v-textarea>
              </v-col>
            </v-row>

            <v-row>
              <v-col cols="12">
                <v-textarea
                  :model-value="formData.accountsNumbers.join('\n')"
                  label="Numery kont"
                  variant="outlined"
                  readonly
                  rows="3"
                  hint="Każdy numer konta w osobnej linii"
                ></v-textarea>
              </v-col>
            </v-row>
          </div>
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
          :disabled="!dataFetched"
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

const emit = defineEmits<{
  'navigate-to-list': []
}>();

interface ContractorFormData {
  name: string;
  nip: string;
  vatStatus: string;
  regon: string;
  krs: string;
  residenceAddress: string;
  accountsNumbers: string[];
}

const form = ref();
const formValid = ref(false);
const loading = ref(false);
const fetchingData = ref(false);
const dataFetched = ref(false);

const formData = reactive<ContractorFormData>({
  name: '',
  nip: '',
  vatStatus: '',
  regon: '',
  krs: '',
  residenceAddress: '',
  accountsNumbers: []
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

const fetchContractorData = async () => {
  if (!validateNIP(formData.nip)) return;
  
  fetchingData.value = true;
  
  try {
    // TODO: Replace with actual Tauri API call
    console.log('Fetching contractor data for NIP:', formData.nip);
    
    // Mock API call - simulate delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Mock response data
    const mockData = {
      name: 'Przykładowa Firma Sp. z o.o.',
      vatStatus: 'Aktywny',
      regon: '123456789',
      krs: '0000123456',
      residenceAddress: 'ul. Przykładowa 123\n00-001 Warszawa\nPolska',
      accountsNumbers: ['12 3456 7890 1234 5678 9012 3456', '98 7654 3210 9876 5432 1098 7654']
    };
    
    // Update form data
    formData.name = mockData.name;
    formData.vatStatus = mockData.vatStatus;
    formData.regon = mockData.regon;
    formData.krs = mockData.krs;
    formData.residenceAddress = mockData.residenceAddress;
    formData.accountsNumbers = mockData.accountsNumbers;
    
    dataFetched.value = true;
    
  } catch (error) {
    console.error('Error fetching contractor data:', error);
    alert('Wystąpił błąd podczas pobierania danych kontrahenta');
  } finally {
    fetchingData.value = false;
  }
};

const handleSubmit = async () => {
  if (!dataFetched.value) return;
  
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

const handleNipEnter = () => {
  if (!dataFetched.value) {
    fetchContractorData();
  }
};

const handleCancel = () => {
  emit('navigate-to-list');
};

const validateField = () => {
  // Trigger validation for the specific field
  form.value?.validate();
};

const resetForm = () => {
  formData.name = '';
  formData.nip = '';
  formData.vatStatus = '';
  formData.regon = '';
  formData.krs = '';
  formData.residenceAddress = '';
  formData.accountsNumbers = [];
  fieldsTouched.name = false;
  fieldsTouched.nip = false;
  dataFetched.value = false;
  form.value?.resetValidation();
};
</script>

<style scoped>
/* Contractor creation styles */
</style>
