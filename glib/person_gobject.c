#include <glib-object.h>
#include <stdio.h>

/*****************************************************************************/
/*        compile
          gcc person_gobject.c `pkg-config --cflags --libs gobject-2.0`                         */
/*****************************************************************************/

G_DECLARE_FINAL_TYPE(MyPerson, my_person, MY, PERSON, GObject)


struct _MyPerson {
	 GObject parent_instance;
	 gchar *name;
	 gint age;
	 gboolean employed;  
};

enum { PROP_0, PROP_NAME, PROP_AGE, PROP_EMPOYED, N_PROPERTIES };

/*
  array of N_PROPERTIES
  properties belongs to class not instnace

*/

static GParamSpec *obj_properties[N_PROPERTIES] = {NULL,};


// forward declarations


static void my_person_set_property(GObject *object, guint property_id,
                                   const GValue *value, GParamSpec *pspec);

static void my_person_get_property(GObject *object, guint property_id,
                                   GValue *value, GParamSpec *pspec);

static void my_person_finalize(GObject *object);



// define types

G_DEFINE_TYPE(MyPerson, my_person, G_TYPE_OBJECT)

static void my_person_class_init(MyPersonClass *klass) {
	 GObjectClass *object_class = G_OBJECT_CLASS(klass);
	 object_class->set_property = my_person_set_property;
	 object_class->get_property = my_person_get_property;
	 object_class->finalize = my_person_finalize;

	 /*registering  properties*/

	 obj_properties[PROP_NAME] = g_param_spec_string(
		  "name", "Name", "Perons's Name", NULL, G_PARAM_READWRITE);

	 obj_properties[PROP_AGE] =
		  g_param_spec_int("age", "Age", "Person's age",
						   0, 150, 0, G_PARAM_READWRITE);

	 obj_properties[PROP_EMPOYED] =
		  g_param_spec_boolean("employed", "Employed",
							   "Employment status", FALSE,
							   G_PARAM_READWRITE);

	 g_object_class_install_properties(object_class, N_PROPERTIES, obj_properties);
  
}

static void my_person_init(MyPerson *self) {
	 self->name = NULL;
	 self->age = 0;
	 self->employed = FALSE;  
}

static void my_person_set_property(GObject *object, guint property_id,
                                   const GValue *value, GParamSpec *pspec) {
	 MyPerson *self = MY_PERSON(object);

	 switch (property_id) {

	 case PROP_NAME:
		  g_free(self->name);
		  self->name = g_value_dup_string(value);
		  break;

	 case PROP_AGE:
		  self->age = g_value_get_int(value);
		  break;

	 case PROP_EMPOYED:
		  self->employed = g_value_get_boolean(value);
		  break;

	 default:
		  G_OBJECT_WARN_INVALID_PROPERTY_ID(object, property_id, pspec);    
	 }
}


static void my_person_get_property(GObject *object, guint property_id,
                                   GValue *value, GParamSpec *pspec) {
	 MyPerson *self = MY_PERSON(object);

	 switch (property_id) {
	 case PROP_NAME:
		  g_value_set_string(value, self->name);
		  break;                  

     case PROP_AGE:
		  g_value_set_int(value, self->age);
		  break;
          
	 case PROP_EMPOYED:
		  g_value_set_boolean(value, self->employed);
		  break;
	 }  
}

static void my_person_finalize(GObject *object) {
	 MyPerson *self = MY_PERSON(object);
	 g_clear_pointer(&self->name, g_free);
	 G_OBJECT_CLASS(my_person_parent_class)
		  ->finalize(object);
}



int main(int argc, char *argv[]) {
	 MyPerson *p;

	 p = g_object_new(MY_TYPE_PERON, "name", "Nithin", "age", 31,
					  "employed", TRUE, NULL);
	 gchar *name;
	 gint age;
	 gboolean employed;

	 g_object_get(p, "name", &name, "age", &age, "employed", &employed,
				  NULL                                            );

	 printf("NAME: %s age %d, employed %s \n", name , age , employed ? "Yes" : "No");
	 g_free(name);
     g_object_unref(p);
     return 0;
}
