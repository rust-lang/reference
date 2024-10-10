/* On the test summary page, toggles the popup for the uncovered tests. */
function spec_toggle_uncovered(item_index) {
    let el = document.getElementById(`uncovered-${item_index}`);
    const currently_hidden = el.classList.contains('popup-hidden');
    const all = document.querySelectorAll('.uncovered-rules-popup');
    all.forEach(element => {
        element.classList.add('popup-hidden');
    });
    if (currently_hidden) {
        el.classList.remove('popup-hidden');
    }
}

function spec_toggle_tests(rule_id) {
    let el = document.getElementById(`tests-${rule_id}`);
    const currently_hidden = el.classList.contains('popup-hidden');
    const all = document.querySelectorAll('.tests-popup');
    all.forEach(element => {
        element.classList.add('popup-hidden');
    });
    if (currently_hidden) {
        el.classList.remove('popup-hidden');
    }
}
