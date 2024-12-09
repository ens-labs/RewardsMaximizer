// helpers/category-icon.js
import { helper } from '@ember/component/helper';

export default helper(function categoryIcon([category]) {
  const iconMap = {
    travel: 'fa-plane',
    shopping: 'fa-shopping-cart',
    dining: 'fa-utensils',
    'cash-back': 'fa-coins',
    groceries: 'fa-apple-alt',
    fuel: 'fa-gas-pump',
    other: 'fa-question-circle',
  };

  return iconMap[category] || 'fa-question-circle';
});
