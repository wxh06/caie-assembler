<script setup lang="ts">
export interface Instruction {
  address: number | "";
  label: string;
  opcode: string;
  operand: string;
}

const props = defineProps<{
  modelValue: Instruction;
  address: number;
  addressMin: number;
  start: number;
  flagged: boolean;
}>();
const emit = defineEmits<{
  (e: "update:modelValue", instruction: Instruction): void;
  (e: "update:start", address: number): void;
  (e: "focus"): void;
}>();

const update = (field: keyof Instruction, value: number | string) =>
  emit("update:modelValue", { ...props.modelValue, [field]: value });
</script>

<script lang="ts">
export const notEmpty = (instruction: Instruction) =>
  Object.values(instruction).map(Boolean).includes(true);
</script>

<template>
  <tr>
    <td>
      <input
        class="form-check-input align-middle"
        type="radio"
        name="start"
        v-if="notEmpty(modelValue)"
        :checked="address === start"
        @input="$emit('update:start', address)"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        :placeholder="`${address}`"
        :class="{ 'is-invalid': flagged }"
        type="number"
        :min="addressMin"
        :value="modelValue.address"
        @input="
          (e) =>
            update(
              'address',
              parseInt((e.target as HTMLInputElement).value, 10),
            )
        "
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        :value="modelValue.label"
        @input="(e) => update('label', (e.target as HTMLInputElement).value)"
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        :value="modelValue.opcode"
        @input="(e) => update('opcode', (e.target as HTMLInputElement).value)"
        @focus="emit('focus')"
      />
    </td>
    <td>
      <input
        class="form-control form-control-sm"
        :value="modelValue.operand"
        @input="(e) => update('operand', (e.target as HTMLInputElement).value)"
        @focus="emit('focus')"
      />
    </td>
  </tr>
</template>
