var $ = mdui.$;
$('#search').on('keyup', function (e) {
    if (e.keyCode === 13) {
        let base_url = window.location.origin;
        window.location.href =
            base_url + '/search?keyword=' + encodeURIComponent($(this).val());
    }
});

function refresh() {
    let full_url = window.location.href;
    window.location.href = full_url;
}

var inst = new mdui.Drawer('#drawer');
function toggle_drawer() {
    inst.toggle();
}
