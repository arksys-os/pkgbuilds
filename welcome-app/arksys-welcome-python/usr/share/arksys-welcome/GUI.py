# =================================================================
# =          Authors: Brad Heffernan & Erik Dubois                =
# =================================================================

import os
import getpass
from os.path import expanduser

DEBUG = False
#DEBUG = True

base_dir = os.path.dirname(os.path.realpath(__file__))
home = expanduser("~")
username = getpass.getuser()

if DEBUG:
    user = username
else:
    user = "arklive"

Settings = home + "/.config/arksys-welcome/settings.conf"
Skel_Settings = "/etc/skel/.config/arksys-welcome/settings.conf"
dot_desktop = "/usr/share/applications/arksys-welcome.desktop"
autostart = home + "/.config/autostart/arksys-welcome.desktop"


def GUI(self, Gtk, GdkPixbuf):

    autostart = eval(self.load_settings())

    self.vbox = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=0)
    self.add(self.vbox)

    hbox1 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox2 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox3 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox4 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox5 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox6 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox7 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox8 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
    hbox9 = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)

    # vbox1 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
    # vbox2 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)

    infoE = Gtk.EventBox()
    pbinfo = GdkPixbuf.Pixbuf().new_from_file_at_size(
        os.path.join(base_dir, 'images/question.png'), 38, 38)
    infoimage = Gtk.Image().new_from_pixbuf(pbinfo)
    infoE.add(infoimage)
    infoE.connect("button_press_event", self.on_info_clicked)
    infoE.set_property("has-tooltip", True)
    infoE.connect("query-tooltip", self.tooltip_callback, "Conflicts Info")

    # ======================================================================
    #                   WELCOME LABEL
    # ======================================================================

    self.cc = Gtk.Label()

    label = Gtk.Label(xalign=0)
    label.set_markup(
        "<big>Welcome to <b>ArcoLinux</b></big>")
    label.set_line_wrap(True)

    # pixbuf = GdkPixbuf.Pixbuf().new_from_file_at_size(
    #     os.path.join(base_dir, 'images/arcolinux-one-liner.png'), 145, 145)
    # image = Gtk.Image().new_from_pixbuf(pixbuf)

    label2 = Gtk.Label(xalign=0)
    label2.set_justify(Gtk.Justification.CENTER)
    label2.set_line_wrap(True)

    label_warning = Gtk.Label(xalign=0)
    label_warning.set_justify(Gtk.Justification.CENTER)
    label_warning.set_line_wrap(True)

    if username == user:

        label2.set_markup(
            "We advise to clean the computer with <b>Gparted</b> before installing.\n" +
            "During the Calamares installation many options will be open to you. You have the freedom of choice.\n" +  # noqa
            "We communicate with our community via a diversity of social media." +  # noqa
            "Do join us to learn the latest news, ask questions or for casual talk.\n" +  # noqa
            "<b>Telegram</b> is for chitchat - <b>Discord</b> is for assistance.\n" +  # noqa
            "We have a <b>forum</b> for the longer and more technical questions.\n")
        label_warning.set_markup(
            "\n<span size='x-large'><b>Use the Easy Installation\n" + # noqa
            "if the Advanced Installation fails</b></span>\n")  # noqa
    else:
        label2.set_markup("The links below will get you started on ArcoLinux. We communicate with our community via a diversity of social media.\n"
                          "Do join us to learn the latest news, ask questions or for casual talk.\n" +  # noqa
                          "<b>Telegram</b> is for chitchat - <b>Discord</b> is for assistance.\n" +  # noqa
                          "We have a <b>forum</b> for the longer and more technical questions.\n" +  # noqa
                          "Learn, have fun and enjoy.")

    hbox4.set_center_widget(label2)
    hbox1.pack_start(label, False, False, 0)
    hbox1.pack_end(self.cc, False, False, 0)
    #hbox4.pack_start(label2, False, False, 0)
    hbox8.pack_start(label_warning, True, False, 0)

    # ======================================================================
    #                   MAIN BUTTONS
    # ======================================================================

    button1 = Gtk.Button(label="")
    button1_label = button1.get_child()
    button1_label.set_markup("<span size='large'><b>Run GParted</b></span>")
    button1.connect("clicked", self.on_gp_clicked)
    button1.set_size_request(0, 80)

    button2 = Gtk.Button(label="")
    button2_label = button2.get_child()
    button2_label.set_markup("<span size='large'><b>Easy Installation (Offline)</b></span>")

    button2.connect("clicked", self.on_ai_clicked)
    button2.set_size_request(0, 80)

    buttonca = Gtk.Button(label="")
    buttonca_label = buttonca.get_child()
    buttonca_label.set_markup("<span size='large'><b>Advanced Installation (Online)</b></span>")

    buttonca.connect("clicked", self.on_aica_clicked)
    buttonca.set_size_request(0, 80)

    self.button8 = Gtk.Button(label="")
    button8_label = self.button8.get_child()
    button8_label.set_markup("<span size='large'><b>Update Arch Linux mirrors</b></span>")
    self.button8.connect("clicked", self.on_mirror_clicked)
    self.button8.set_size_request(420, 70)

    self.buttonpamac = Gtk.Button(label="")
    buttonpamac_label = self.buttonpamac.get_child()
    buttonpamac_label.set_markup("<span size='large'><b>Install software</b></span>")
    self.buttonpamac.connect("clicked", self.on_buttonpamac_clicked)
    self.buttonpamac.set_size_request(420, 70)

    # grid.add(button1)
    if username == user:
        grid = Gtk.Grid()
        grid.attach(self.button8, 2, 0, 2, 2)
        #grid.attach(button13, 2, 0, 2, 2)
        grid.attach(button1, 2, 2, 2, 2)
        grid.attach(button2, 1, 4, 2, 2)
        grid.attach(buttonca, 3, 4, 2, 2)
        grid.set_column_homogeneous(True)
        grid.set_row_homogeneous(True)
    else:
        grid = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        self.button8.set_size_request(300, 70)
        self.buttonatt.set_size_request(300, 70)
        self.buttonpamac.set_size_request(300, 70)
        grid.pack_start(self.buttonpamac, True, False, 0)
        grid.pack_start(self.buttonatt, True, False, 0)
        grid.pack_start(self.button8, True, False, 0)
    # grid.set_row_homogeneous(True)

    # ======================================================================
    #                   NOTICE
    # ======================================================================

    # label3 = Gtk.Label(xalign=0)
    # label3.set_line_wrap(True)

    # label4 = Gtk.Label(xalign=0)
    # label4.set_line_wrap(True)

    # self.vbox2 = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)

    # self.vbox2.pack_start(label3, False,False,0)
    # self.vbox2.pack_start(label4, False,False,0)

    # ======================================================================
    #                   USER INFO
    # ======================================================================

    lblusrname = Gtk.Label(xalign=0)
    lblusrname.set_text("User:")

    lblpassword = Gtk.Label(xalign=0)
    lblpassword.set_text("Pass:")

    lblusr = Gtk.Label(xalign=0)
    lblusr.set_text("arklive  |")

    lblpass = Gtk.Label(xalign=0)
    lblpass.set_markup("<i>No Password</i>")

    hboxUser = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)

    hboxUser.pack_start(lblusrname, False, False, 0)
    hboxUser.pack_start(lblusr, False, False, 0)

    hboxUser.pack_start(lblpassword, False, False, 0)
    hboxUser.pack_start(lblpass, False, False, 0)

    # ======================================================================
    #                   FOOTER BUTTON LINKS
    # ======================================================================

    button12 = Gtk.Button(label="Quit")
    button12.set_size_request(200, 50)
    button12.connect("clicked", Gtk.main_quit)
    #button12.set_tooltip_markup("Quit the Arksys Welcome")

    hbox5.pack_start(button8, True, True, 0)
    hbox5.pack_start(button9, True, True, 0)
    hbox5.pack_start(button10, True, True, 0)
    hbox5.pack_start(button11, True, True, 0)
    hbox5.pack_start(button12, True, True, 0)


    # hbox8.pack_start(self.button8, True, False, 0)

    # ======================================================================
    #                   Add to startup
    # ======================================================================

    check = Gtk.CheckButton(label="Autostart")
    check.connect("toggled", self.statup_toggle)
    check.set_active(autostart)
    hbox3.pack_end(check, False, False, 0)

    # ======================================================================
    #                   SOCIAL LINKS
    # ======================================================================
    tgE = Gtk.EventBox()

    pbtg = GdkPixbuf.Pixbuf().new_from_file_at_size(
        os.path.join(base_dir, 'images/tg.png'), 28, 28)
    tgimage = Gtk.Image().new_from_pixbuf(pbtg)

    tgE.add(tgimage)

    tgE.connect("button_press_event", self.on_social_clicked,
                "https://github.com/arksys-os")

    tgE.set_property("has-tooltip", True)

    tgE.connect("query-tooltip", self.tooltip_callback, "Telegram")

    hbox3.pack_start(fbE, False, False, 0)
    hbox3.pack_start(tE, False, False, 0)
    hbox3.pack_start(meE, False, False, 0)
    hbox3.pack_start(inE, False, False, 0)
    hbox3.pack_start(liE, False, False, 0)
    hbox3.pack_start(elE, False, False, 0)

    hbox6.pack_start(pE, False, False, 50)
    hbox6.pack_start(yE, False, False, 0)
    hbox6.pack_start(dE, False, False, 0)
    hbox6.pack_start(tgE, False, False, 0)
    if username == user:
        hbox3.pack_start(hboxUser, True, False, 0)
    hbox3.pack_start(hbox6, True, False, 0)

    # ======================================================================
    #                   Start Arcolinux Tweak Tool
    # ======================================================================
    launchBox = Gtk.EventBox()
    pblaunch = GdkPixbuf.Pixbuf().new_from_file_at_size(
        os.path.join(base_dir, 'images/archlinux-tweak-tool.svg'), 40, 40)
    launchimage = Gtk.Image().new_from_pixbuf(pblaunch)

    launchBox.add(launchimage)
    launchBox.connect("button_press_event", self.on_launch_clicked, "")

    launchBox.set_property("has-tooltip", True)
    launchBox.connect("query-tooltip",
                      self.tooltip_callback,
                      "Launch Arcolinux Tweak Tool")

    hbox6.pack_start(launchBox, False, False, 0)
    #hbox6.pack_start(infoE, False, False, 0)
    # ======================================================================
    #                   PACK TO WINDOW
    # ======================================================================
    label3 = Gtk.Label("v20.6-4")
    hbox7.pack_end(label3, False, False, 0)
    # if self.is_connected():
    #     self.get_message(label3, label4)

    self.vbox.pack_start(hbox1, False, False, 7)  # Logo
    self.vbox.pack_start(hbox4, False, False, 7)  # welcome Label
    self.vbox.pack_start(hbox8, False, False, 7)  # warning Label

    self.vbox.pack_start(grid, True, False, 7)  # Run GParted/Calamares

    # if self.results and self.is_connected():
    #     self.vbox.pack_start(self.vbox2, False, False, 0)  # Notice

    self.vbox.pack_end(hbox3, False, False, 0)  # Footer
    #self.vbox.pack_end(hbox7, False, False, 0)  # Version
    self.vbox.pack_end(hbox5, False, False, 7)  # Buttons
    self.vbox.pack_end(hbox2, False, False, 7)  # Buttons
